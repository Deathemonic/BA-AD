from argparse import ArgumentParser, ArgumentTypeError
from pathlib import Path

from rich.console import Console
from rich.traceback import Traceback

from . import __version__
from .extractors.assetextractor import AssetExtractor
from .utils.flatgen import FlatbufGenerator
from .utils.downloader import ResourceDownloader
from .extractors.studioextractor import AssetStudioExtractor
from .extractors.tableextractor import TableExtractor
from .extractors.mediaextractor import MediaExtractor
from .utils.list import List
from .utils.apk import Apk
from .utils.parser import CatalogParser


def arguments() -> tuple:
    parser = ArgumentParser(description='Blue Archive Asset Downloader')
    sub_parser = parser.add_subparsers(dest='commands')

    parser.add_argument(
        '-v',
        '--version',
        action='version',
        version=f'baad {__version__}',
    )
    parser.add_argument(
        '-u',
        '--update',
        action='store_true',
        help='force update the apk',
    )
    parser.add_argument(
        '-g',
        '--generate',
        action='store_true',
        help='generate the flatbuf schemas (this will be removed in the future and will be replaced by BA-FB)',
    )

    search = sub_parser.add_parser(
        'search',
        help='search mode',
    )
    search.add_argument(
        '--output',
        type=str,
        help='output directory for the downloaded files (default: ./output)',
    )

    download = sub_parser.add_parser(
        'download',
        help='download game files',
    )
    download.add_argument(
        '--output',
        type=str,
        help='output directory for the downloaded files (default: ./output)',
    )
    download.add_argument(
        '--limit',
        type=int,
        default=5,
        help='set a limit the download limit (default: 5)',
    )
    download.add_argument(
        '--filter',
        type=str,
        help='filter by name',
    )
    download.add_argument(
        '--assets',
        action='store_true',
        help='download the assetbundles',
    )
    download.add_argument(
        '--tables',
        action='store_true',
        help='download the tablebundles',
    )
    download.add_argument(
        '--media',
        action='store_true',
        help='download the mediaresources',
    )
    download.add_argument(
        '-a',
        '--all',
        action='store_true',
        help='download all game files',
    )

    extract = sub_parser.add_parser(
        'extract',
        help='extract game files (this will be removed in the future and will be replaced by BA-AE)',
    )
    extract.add_argument(
        '--path',
        type=str,
        help='path of the files that will be extracted',
    )
    extract.add_argument(
        '--studio',
        action='store_true',
        help='uses the assetstudiomod as a backend for extracting the assetbundles',
    )
    extract.add_argument(
        '--assets',
        action='store_true',
        help='extract the assetbundles',
    )
    extract.add_argument(
        '--tables',
        action='store_true',
        help='extract the tablebundles',
    )
    extract.add_argument(
        '--media',
        action='store_true',
        help='extract the mediaresources',
    )
    extract.add_argument(
        '-a',
        '--all',
        action='store_true',
        help='extract all game files',
    )

    args = parser.parse_args()

    if (
        hasattr(args, 'commands')
        and args.commands in ['download', 'extract']
        and (args.all and (args.assets or args.tables or args.media))
    ):
        console = Console(stderr=True)
        console.print(
            Traceback.from_exception(
                type(ArgumentTypeError),
                ArgumentTypeError("'--all' cannot be used with other download options"),
                None,
            )
        )
        raise SystemExit(1)

    if hasattr(args, 'commands') and args.commands == 'extract' and sum([args.assets, args.tables, args.media]) > 1:
        console = Console(stderr=True)
        console.print(
            Traceback.from_exception(
                type(ArgumentTypeError),
                ArgumentTypeError("Cannot use multiple extract options together (--assets, --tables, --media)"),
                None,
            )
        )
        raise SystemExit(1)
    return parser, args


def handle_apk(args, console) -> str:
    apk = Apk()
    
    if args.update:
        console.print("[yellow]Force updating APK...[/yellow]")
        apk.download_apk(update=True)
    else:
        if not apk.apk_exists():
            console.print("[yellow]APK doesn't exist. Downloading...[/yellow]")
            apk.download_apk()
        elif apk.is_outdated():
            console.print("[yellow]APK is outdated. Updating...[/yellow]")
            apk.download_apk()
        else:
            console.print("[green]APK is up to date.[/green]")
    
    parser = CatalogParser()
    return parser.fetch_catalog_url()


def handle_download(args, catalog_url) -> None:
    downloader = ResourceDownloader(
        update=args.update,
        output=args.output,
        filter_pattern=args.filter if hasattr(args, 'filter') else None
    )

    if args.all:
        args.assets = args.tables = args.media = True

    limit = None if args.limit == 0 else args.limit
    downloader.download(
        assets=args.assets,
        tables=args.tables,
        media=args.media,
        limit=limit
    )


def handle_extract(args) -> None:
    if args.all:
        args.tables = True
        args.assets = True
        args.media = True

    if args.tables:
        table_extract = TableExtractor(args.path)
        table_extract.run_extraction()

    if args.assets and not args.studio:
        asset_extract = AssetExtractor(args.path)
        asset_extract.extract_assets()

    if args.assets and args.studio:
        asset_studio_extract = AssetStudioExtractor(args.path)
        asset_studio_extract.extract_assets()

    if args.media:
        media_extract = MediaExtractor(args.path)
        media_extract.run_extraction()


def handle_search(args) -> None:
    root_path = Path(__file__).parent
    output_path = args.output if hasattr(args, 'output') and args.output else None
    
    catalog_list = List(root_path, update=args.update, output=output_path)
    catalog_list.show()


def main() -> None:
    parser, args = arguments()
    console = Console()

    if not hasattr(args, 'commands'):
        parser.print_help()
        return

    if args.generate:
        FlatbufGenerator().generate()
        return

    catalog_url = None
    if args.commands in ['download', 'search']:
        catalog_url = handle_apk(args, console)

    if args.commands == 'download':
        handle_download(args, catalog_url)
    elif args.commands == 'extract':
        handle_extract(args)
    elif args.commands == 'search':
        handle_search(args)


if __name__ == '__main__':
    main()
