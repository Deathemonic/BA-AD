from argparse import ArgumentParser, ArgumentTypeError
from pathlib import Path

from rich.console import Console
from rich.traceback import Traceback

from . import __version__
from .utils.AssetExtracter import AssetExtracter
from .utils.FlatbufGenerator import FlatbufGenerator
from .utils.ResourceDownloader import ResourceDownloader
from .utils.StudioExtracter import AssetStudioExtracter
from .utils.TableExtracter import TableExtracter
from .utils.MediaExtracter import MediaExtracter
from .utils.CatalogList import CatalogList


def arguments() -> tuple:  # sourcery skip: extract-duplicate-method
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
        help='generate the flatbuf schemas',
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
    search.add_argument(
        '--version',
        type=str,
        help='specific version to search for (default: latest)',
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
        '--version',
        type=str,
        help='specific version to download (default: latest)',
    )
    download.add_argument(
        '--limit',
        type=int,
        default=5,
        help='set a limit the download limit (default: 5)',
    )
    download.add_argument(
        '--catalog',
        type=str,
        help='force change the catalog url (will skip apk download)',
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
        help='extract game files',
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


def resource_downloader(args) -> ResourceDownloader:
    downloader_args = {
        'update': args.update,
        'catalog_url': args.catalog,
        'filter_pattern': args.filter if hasattr(args, 'filter') else None,
        'version': args.version if hasattr(args, 'version') else None
    }
    
    if args.output:
        downloader_args['output'] = args.output
    downloader = ResourceDownloader(**downloader_args)

    if hasattr(args, 'commands') and args.commands == 'download':
        if args.all:
            args.assets = args.tables = args.media = True

        limit = None if args.limit == 0 else args.limit
        downloader.download(
            assets=args.assets,
            tables=args.tables,
            media=args.media,
            limit=limit,
        )
    return downloader


def extracter(args) -> TableExtracter | AssetExtracter | AssetStudioExtracter | MediaExtracter | None:
    if args.all:
        args.tables = True
        args.assets = True
        args.media = True

    if args.tables:
        table_extract = TableExtracter(args.path)
        table_extract.run_extraction()
        return table_extract

    if args.assets and not args.studio:
        asset_extract = AssetExtracter(args.path)
        asset_extract.extract_assets()
        return asset_extract

    if args.assets and args.studio:
        asset_studio_extract = AssetStudioExtracter(args.path)
        asset_studio_extract.extract_assets()
        return asset_studio_extract

    if args.media:
        media_extract = MediaExtracter(args.path)
        media_extract.run_extraction()
        return media_extract

    return None


def main() -> None:
    parser, args = arguments()

    if not hasattr(args, 'commands'):
        return

    if args.commands == 'download':
        resource_downloader(args)
        return
        
    if args.commands == 'extract':
        extracter(args)
        return
        
    if args.commands == 'search':
        root_path = Path(__file__).parent
        output_path = args.output if hasattr(args, 'output') else None
        catalog_list = CatalogList(root_path, output_path)
        catalog_list.show()
        return

    if args.update:
        ResourceDownloader(update=args.update).fetch_catalog_url()
        return

    if args.generate:
        FlatbufGenerator().generate()
        return

    parser.print_help()


if __name__ == '__main__':
    main()
