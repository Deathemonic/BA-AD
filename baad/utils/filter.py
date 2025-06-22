from pathlib import Path

from rapidfuzz import process, fuzz
from ..helpers.json import load_json


class Filter:
    def __init__(self, game_files_path: Path):
        self.game_files_path = game_files_path
        self.score_cutoff = 85

    def _load_game_files(self) -> dict:
        return load_json(self.game_files_path)

    def _find_matches(self, pattern: str, choices: list) -> list:
        pattern = pattern.lower()
        
        matches = [
            item for item in choices 
            if pattern in str(item['Url']).lower()
        ]

        if not matches:
            matches = [
                item for item in choices
                if process.extractOne(
                    query=pattern,
                    choices=[str(item['Url'])], 
                    scorer=fuzz.token_sort_ratio,
                    score_cutoff=self.score_cutoff
                )
            ]

        return matches

    def filter_files(self, pattern: str) -> dict:
        game_files = self._load_game_files()
        
        asset_matches = self._find_matches(pattern, game_files.get('AssetBundles', []))
        asset_results = [
            {
                'Url': asset['Url'],
                'Crc': asset['Crc'],
                'Size': asset.get('Size', 0),
                'Name': Path(asset['Url']).name
            }
            for asset in asset_matches
        ]

        table_matches = self._find_matches(pattern, game_files.get('TableBundles', []))
        table_results = [
            {
                'Url': table['Url'],
                'Crc': table['Crc'],
                'Size': table.get('Size', 0),
                'Name': Path(table['Url']).name
            }
            for table in table_matches
        ]

        media_matches = self._find_matches(pattern, game_files.get('MediaResources', []))
        media_results = [
            {
                'Url': media['Url'],
                'Path': media['Path'],
                'Crc': media['Crc'],
                'Size': media.get('Size', 0),
                'Name': Path(media['Path']).name
            }
            for media in media_matches
        ]

        return {
            'AssetBundles': asset_results,
            'TableBundles': table_results, 
            'MediaResources': media_results
        }