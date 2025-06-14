import json
from collections import Counter
from pathlib import Path
from typing import Callable

import UnityPy

from ..helpers.progress import create_live_display, create_progress_group
from ..helpers.filemanager import ensure_directory_exists, get_asset_output_dir, get_extracted_dir


# TODO: Deprecated (Moved to BA-AE)
class AssetExtractor:
    def __init__(self, output_path: Path) -> None:
        self.asset_path = get_asset_output_dir(output_path)
        self.ignore_count = 0
        self.types = {
            'Sprite',
            'Texture2D',
            'TextAsset',
            'Shader',
            'MonoBehaviour',
            'Mesh',
            'Font',
            'AudioClip',
        }

        self.live = create_live_display()
        self.progress_group, _, self.extract_progress, self.print_progress, self.console = create_progress_group()

    def _sort_objects(self, objects) -> list:
        return sorted(
            (obj for obj in objects if obj.type.name in self.types),
            key=lambda x: 1 if x.type == 'Texture2D' else 0,
        )

    def _sort_container_objects(self, container_items) -> list:
        return sorted(
            ((key, obj) for key, obj in container_items if obj.type.name in self.types),
            key=lambda x: 1 if x[1].type == 'Texture2D' else 0,
        )

    def _extract_objects(self, objs, cobjs, asset) -> None:
        num_cont, num_objs = len(cobjs), len(objs)

        if num_objs <= num_cont * 2:
            self._extract_from_container(cobjs)
            return

        self._extract_from_objects(objs, asset)

    def _extract_from_objects(self, objs, asset) -> None:
        local_path = self._get_most_common_path(asset)
        [self._try_export_obj(obj, local_path) for obj in objs]

    def _get_most_common_path(self, asset) -> Path:
        occurrence_count = Counter(Path(asset_path).with_suffix('') for asset_path in asset.container.keys())
        export_path = get_extracted_dir(Path(self.asset_path).parent, 'Asset')

        return export_path.joinpath(*occurrence_count.most_common(1)[0][0].parts[self.ignore_count :])

    def _extract_from_container(self, cobjs) -> None:
        [self._export_obj(obj, self._get_file_path(asset_path)) for asset_path, obj in cobjs]

    def _try_export_obj(self, obj, local_path: Path) -> None:
        try:
            self._export_obj(obj, local_path, append_name=True)

        except Exception as e:
            self.console.log(e, obj.path_id)

    def _export_obj(self, obj, fp: Path, append_name: bool = False) -> list:
        if obj.type.name not in self.types:
            return []

        data = obj.read()
        fp = self._prepare_file_path(fp, data, append_name)

        export_func = self._get_export_function(obj.type.name)
        return export_func(obj, data, fp)

    def _get_file_path(self, asset_path: str) -> Path:
        export_path = get_extracted_dir(Path(self.asset_path).parent, 'Asset')
        return export_path.joinpath(*Path(asset_path).parts[self.ignore_count :])

    def _prepare_file_path(self, fp: Path, data, append_name: bool) -> Path:
        if append_name:
            fp = fp / data.name
        ensure_directory_exists(fp.parent)

        return fp

    def _get_export_function(self, obj_type: str) -> Callable:
        export_functions = {
            'TextAsset': self._export_text_asset,
            'Font': self._export_font,
            'Mesh': self._export_mesh,
            'Shader': self._export_shader,
            'MonoBehaviour': self._export_mono_behaviour,
            'Sprite': self._export_sprite,
            'Texture2D': self._export_texture2d,
            'AudioClip': self._export_audio_clip,
        }
        return export_functions.get(obj_type, lambda *args: [])

    def _export_text_asset(self, obj, data, fp: Path) -> list:
        fp.with_suffix('.txt').write_bytes(data.script)
        return [obj.path_id]

    def _export_font(self, obj, data, fp: Path) -> list:
        if not data.m_FontData:
            return [obj.path_id]

        suffix = '.otf' if data.m_FontData[:4] == b'OTTO' else '.ttf'
        fp.with_suffix(suffix).write_bytes(data.m_FontData)

        return [obj.path_id]

    def _export_mesh(self, obj, data, fp: Path) -> list:
        fp.with_suffix('.obj').write_bytes(data.export().encode('utf-8'))
        return [obj.path_id]

    def _export_shader(self, obj, data, fp: Path) -> list:
        fp.with_suffix('.txt').write_bytes(data.export().encode('utf-8'))
        return [obj.path_id]

    def _export_mono_behaviour(self, obj, data, fp: Path) -> list:
        if obj.serialized_type.nodes:
            export = json.dumps(obj.read_typetree(), indent=4, ensure_ascii=False).encode('utf-8')

            fp.with_suffix('.json').write_bytes(export)
        fp.with_suffix('.bin').write_bytes(data.raw_data)

        return [obj.path_id]

    def _export_sprite(self, obj, data, fp: Path) -> list:
        data.image.save(str(fp.with_suffix('.tga')))
        return [obj.path_id, data.m_RD.texture.path_id, getattr(data.m_RD.alphaTexture, 'path_id', None)]

    def _export_texture2d(self, obj, data, fp: Path) -> list:
        if not fp.exists() and data.m_Width:
            data.image.save(str(fp.with_suffix('.tga')))
        return [obj.path_id]

    def _export_audio_clip(self, obj, data, fp: Path) -> list:
        samples = data.samples
        if len(samples) == 1:
            fp.with_suffix('.wav').write_bytes(list(data.samples.values())[0])

        if len(samples) > 1:
            ensure_directory_exists(fp)
            {fp.joinpath(f'{name}.wav').write_bytes(clip_data) for name, clip_data in samples.items()}

        return [obj.path_id]

    def _process_asset(self, asset, task_id) -> None:
        objs = self._sort_objects(asset.get_objects())
        cobjs = self._sort_container_objects(asset.container.items())

        self._extract_objects(objs, cobjs, asset)
        self.extract_progress.update(task_id, advance=1)
        self.live.update(self.progress_group)

    def extract_assets(self) -> None:
        try:
            with self.live:
                env = UnityPy.load(self.asset_path)
                assets = [asset for asset in env.assets if asset.container]

                extract_task = self.extract_progress.add_task('[green]Extracting assets...', total=len(assets))

                for asset in assets:
                    self._process_asset(asset, extract_task)

                self.print_progress.add_task('[green]Asset extraction completed![/green]')
        finally:
            self.live.stop()
