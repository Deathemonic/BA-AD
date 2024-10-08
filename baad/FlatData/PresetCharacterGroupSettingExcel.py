# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class PresetCharacterGroupSettingExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = PresetCharacterGroupSettingExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsPresetCharacterGroupSettingExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # PresetCharacterGroupSettingExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # PresetCharacterGroupSettingExcel
    def CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # PresetCharacterGroupSettingExcel
    def ArenaSimulatorFixed(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # PresetCharacterGroupSettingExcel
    def PresetType(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # PresetCharacterGroupSettingExcel
    def PresetTypeLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # PresetCharacterGroupSettingExcel
    def PresetTypeIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        return o == 0

def Start(builder): builder.StartObject(3)
def PresetCharacterGroupSettingExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddCharacterId(builder, CharacterId): builder.PrependInt64Slot(0, CharacterId, 0)
def PresetCharacterGroupSettingExcelAddCharacterId(builder, CharacterId):
    """This method is deprecated. Please switch to AddCharacterId."""
    return AddCharacterId(builder, CharacterId)
def AddArenaSimulatorFixed(builder, ArenaSimulatorFixed): builder.PrependBoolSlot(1, ArenaSimulatorFixed, 0)
def PresetCharacterGroupSettingExcelAddArenaSimulatorFixed(builder, ArenaSimulatorFixed):
    """This method is deprecated. Please switch to AddArenaSimulatorFixed."""
    return AddArenaSimulatorFixed(builder, ArenaSimulatorFixed)
def AddPresetType(builder, PresetType): builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(PresetType), 0)
def PresetCharacterGroupSettingExcelAddPresetType(builder, PresetType):
    """This method is deprecated. Please switch to AddPresetType."""
    return AddPresetType(builder, PresetType)
def StartPresetTypeVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def PresetCharacterGroupSettingExcelStartPresetTypeVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartPresetTypeVector(builder, numElems)
def End(builder): return builder.EndObject()
def PresetCharacterGroupSettingExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)