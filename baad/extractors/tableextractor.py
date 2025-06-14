import asyncio
import json
from pathlib import Path
from zipfile import BadZipFile

from .. import FlatData
from ..FlatData.dump import dump_table
from ..crypto.cipher import decrypt
from ..helpers.progress import create_live_display, create_progress_group
from ..helpers.filemanager import ensure_directory_exists, get_table_output_dir, get_extracted_dir
from bacy import TableZipFile, xor

# TODO: Deprecated (Moved to BA-AE)
class TableExtractor:
    def __init__(self, output: str) -> None:
        self.table_path = get_table_output_dir(output)
        self.extracted_path = get_extracted_dir(output, 'Table')
        ensure_directory_exists(self.extracted_path)

        self.lower_name_to_module_dict = self._get_lower_name_to_module_dict()

        self.live = create_live_display()
        self.progress_group, _, self.extract_progress, self.print_progress, self.console = create_progress_group()

    @staticmethod
    def _get_lower_name_to_module_dict() -> dict:
        return {key.lower(): value for key, value in FlatData.__dict__.items()}

    @staticmethod
    def _process_json_file(name: str, data: bytes) -> bytes:
        if name == 'logiceffectdata.json':
            return decrypt(data, 'LogicEffectData').encode('utf-8')
        elif name == 'newskilldata.json':
            return decrypt(data, 'NewSkillData').encode('utf-8')
        return data

    def _process_excel_file(self, name: str, data: bytes) -> tuple:
        flatbuffer_cls = self.lower_name_to_module_dict[name[:-6]]
        data = xor(flatbuffer_cls.__name__, data)
        flatbuffer = flatbuffer_cls.GetRootAs(data)

        processed_data = json.dumps(dump_table(flatbuffer), indent=4, ensure_ascii=False).encode('utf-8')
        new_name = f'{flatbuffer_cls.__name__}.json'

        return processed_data, new_name

    async def extract_table(self, table_file: Path | str, task: int) -> None:
        try:
            zip_file = TableZipFile(str(table_file), None)
            table_dir_fp = self.extracted_path / table_file.stem
            ensure_directory_exists(table_dir_fp)

            try:
                data = zip_file.open(table_file.stem)
                if table_file.name == 'Excel.zip':
                    data, new_name = self._process_excel_file(table_file.stem, data)
                    fp = table_dir_fp / new_name
                else:
                    data = self._process_json_file(table_file.stem, data)
                    fp = table_dir_fp / table_file.stem

                ensure_directory_exists(fp.parent)
                fp.write_bytes(data)

                self.extract_progress.update(task, advance=1)
                self.live.update(self.progress_group)

            except Exception as e:
                self.print_progress.add_task(f'[yellow]Warning processing {table_file.name}: {e}[/yellow]')

        except BadZipFile:
            self.console.log(f'[red]Error: {table_file} is not a valid zip file.[/red]')

    async def extract_all_tables(self) -> None:
        table_files = list(Path(self.table_path).glob('*.zip'))
        extract_task = self.extract_progress.add_task('[green]Extracting...', total=len(table_files))

        task = [self.extract_table(table_file, extract_task) for table_file in table_files]
        await asyncio.gather(*task)
        
        self.extract_progress.update(extract_task, description='[green]Extracted...')
        self.print_progress.add_task('[green]Extraction completed![/green]')

    def run_extraction(self) -> None:
        try:
            with self.live:
                asyncio.run(self.extract_all_tables())

        finally:
            self.live.stop()
