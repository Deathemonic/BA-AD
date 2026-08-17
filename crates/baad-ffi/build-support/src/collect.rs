use syn::Item;

use crate::config::{Config, RemoteKind, Source};
use crate::consts::{ConstField, GeneratedConst};
use crate::context::{
    Context,
    DataMirror,
    ErrorCode,
    ErrorMirror,
    Remote,
    RemoteItem,
    SanitizedStruct
};
use crate::mirror::enum_is_mirrorable;
use crate::syntax::{
    enum_is_ffi_safe,
    fields_are_named,
    is_public,
    parse_file,
    rename,
    rewrite_enum,
    rewrite_struct,
    skipped,
    source_dir,
    struct_is_ffi_safe
};

pub fn collect_source(config: &Config, source: &'static Source, context: &mut Context) {
    let base = source_dir(source.dir);

    for (file, kind) in source.type_files {
        let path = base.join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        collect_types(config, source, &parse_file(&path).items, *kind, context);
    }

    for file in source.const_files {
        let path = base.join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        for item in &parse_file(&path).items {
            if let Item::Const(item) = item
                && is_public(&item.vis)
                && let Some(field) = ConstField::from_item(item)
            {
                context.consts.push(GeneratedConst {
                    crate_path: source.crate_path.into(),
                    field
                });
            }
        }
    }

    for spec in source.sanitized {
        let path = base.join(spec.file);
        println!("cargo:rerun-if-changed={}", path.display());
        let item = parse_file(&path)
            .items
            .into_iter()
            .find_map(|item| match item {
                Item::Struct(item) if item.ident == spec.name => Some(item),
                _ => None
            })
            .unwrap_or_else(|| panic!("sanitized struct {} not found in {}", spec.name, spec.file));
        context.sanitized.push(SanitizedStruct { item, spec });
    }
}

fn collect_types(
    config: &Config,
    source: &Source,
    items: &[Item],
    kind: RemoteKind,
    context: &mut Context
) {
    for item in items {
        match item {
            Item::Struct(item)
                if is_public(&item.vis)
                    && !skipped(source, &item.ident)
                    && fields_are_named(&item.fields) =>
            {
                let mut rewritten = rewrite_struct(config, item);
                if struct_is_ffi_safe(&rewritten) {
                    rename(source, &mut rewritten.ident);
                    context.remotes.push(Remote {
                        crate_path: source.crate_path.into(),
                        source: item.ident.to_string(),
                        item: RemoteItem::Record(rewritten)
                    });
                }
            }
            Item::Enum(item) if is_public(&item.vis) && !skipped(source, &item.ident) => {
                let mut rewritten = rewrite_enum(config, item);
                rename(source, &mut rewritten.ident);
                match kind {
                    RemoteKind::Data if enum_is_ffi_safe(&rewritten) => {
                        context.remotes.push(Remote {
                            crate_path: source.crate_path.into(),
                            source: item.ident.to_string(),
                            item: RemoteItem::Enum(rewritten)
                        });
                    }
                    RemoteKind::Data if enum_is_mirrorable(item) => {
                        context.mirrored_names.push(item.ident.to_string());
                        context.data_mirrors.push(DataMirror {
                            crate_path: source.crate_path.into(),
                            item: item.clone()
                        });
                    }
                    RemoteKind::Data => {}
                    RemoteKind::Error => {
                        context.error_codes.push(ErrorCode {
                            crate_path: source.crate_path.into(),
                            item: item.clone()
                        });
                        if enum_is_ffi_safe(&rewritten) {
                            context.remotes.push(Remote {
                                crate_path: source.crate_path.into(),
                                source: item.ident.to_string(),
                                item: RemoteItem::Error(rewritten)
                            });
                        } else {
                            context.error_mirrors.push(ErrorMirror {
                                crate_path: source.crate_path.into(),
                                item: item.clone()
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
