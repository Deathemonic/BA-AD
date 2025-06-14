import json
from pathlib import Path
from typing import Dict, Any


def load_json(file_path: Path) -> Dict[str, Any]:
    with open(file_path, 'r') as f:
        return json.load(f)


def save_json(file_path: Path, data: Dict[str, Any], indent: int = 4) -> None:
    with open(file_path, 'w') as f:
        json.dump(data, f, indent=indent)
