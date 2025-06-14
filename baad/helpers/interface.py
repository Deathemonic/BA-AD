import sys
from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.table import Table
from rich.text import Text


def format_size(size: int) -> str:
    if size == 0:
        return ""
    
    for unit in ['B', 'KB', 'MB', 'GB']:
        if size < 1024:
            return f"{size:.1f}{unit}"
        size /= 1024
    
    return f"{size:.1f}TB"


def create_table(columns: list, rows: list, header_style: str = "bold magenta", expand: bool = True) -> Table:
    table = Table(show_header=True, header_style=header_style, expand=expand, box=None)
    
    for col_name, col_style, col_justify, col_width in columns:
        table.add_column(col_name, style=col_style, justify=col_justify, width=col_width)
    
    for row in rows:
        table.add_row(*row)
        
    return table


def create_layout() -> Layout:
    layout = Layout()
    layout.split_column(
        Layout(name="search", size=3),
        Layout(name="content")
    )
    return layout


def create_panel(content, title: str = "", style: str = "") -> Panel:
    return Panel(content, title=title, style=style)


def get_character_input() -> str:
    if sys.platform == "win32":
        import msvcrt
        if not msvcrt.kbhit():
            return ""
        
        char = msvcrt.getch()

        if char in [b'\x03', b'\x1b']:  # Ctrl+C, ESC
            return '\x1b'
        
        elif char == b'\x08':  # Backspace
            return '\x08'
        
        elif char == b'\xe0':
            char = msvcrt.getch()

            if char == b'H':  # Up arrow
                return '\x1b[A'
            
            elif char == b'P':  # Down arrow
                return '\x1b[B'
            
        return char.decode('utf-8', errors='ignore')
    
    elif sys.platform == "posix":
        import tty
        import termios
        import select
        
        if not select.select([sys.stdin], [], [], 0)[0]:
            return ""
            
        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd)

        try:
            tty.setraw(sys.stdin.fileno())
            char = sys.stdin.read(1)
            
            if char == '\x1b':  # ESC sequence
                if select.select([sys.stdin], [], [], 0.1)[0]:
                    char += sys.stdin.read(2)

        finally:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
            
        return char
    
    return ""
