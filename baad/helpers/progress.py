from rich.console import Console, Group
from rich.live import Live
from rich.progress import BarColumn, DownloadColumn, Progress, TextColumn, TimeRemainingColumn, TransferSpeedColumn


def create_progress_group() -> tuple:
    console = Console()

    print_progress = Progress(TextColumn('{task.description}'), console=console)

    download_progress = Progress(
        TextColumn('[bold blue]{task.description}', justify='right'),
        BarColumn(bar_width=None, finished_style='green'),
        '[progress.percentage]{task.percentage:>3.1f}%',
        '•',
        DownloadColumn(),
        '•',
        TransferSpeedColumn(),
        '•',
        TimeRemainingColumn(),
        expand=True,
    )

    extract_progress = Progress(
        TextColumn('[bold green]{task.description}', justify='right'),
        BarColumn(bar_width=None, finished_style='green'),
        '[progress.percentage]{task.percentage:>3.1f}%',
        '•',
        TimeRemainingColumn(),
        expand=True,
    )

    progress_group = Group(print_progress, download_progress, extract_progress)

    return progress_group, download_progress, extract_progress, print_progress, console


def create_live_display() -> Live:
    return Live(create_progress_group()[0], refresh_per_second=10)
