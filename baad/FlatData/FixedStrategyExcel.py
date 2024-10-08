# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FixedStrategyExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FixedStrategyExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFixedStrategyExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FixedStrategyExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FixedStrategyExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon01FixedEchelonId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon01Starttile(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon02FixedEchelonId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon02Starttile(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon03FixedEchelonId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon03Starttile(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon04FixedEchelonId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FixedStrategyExcel
    def StageEnterEchelon04Starttile(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(9)
def FixedStrategyExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def FixedStrategyExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddStageEnterEchelon01FixedEchelonId(builder, StageEnterEchelon01FixedEchelonId): builder.PrependInt64Slot(1, StageEnterEchelon01FixedEchelonId, 0)
def FixedStrategyExcelAddStageEnterEchelon01FixedEchelonId(builder, StageEnterEchelon01FixedEchelonId):
    """This method is deprecated. Please switch to AddStageEnterEchelon01FixedEchelonId."""
    return AddStageEnterEchelon01FixedEchelonId(builder, StageEnterEchelon01FixedEchelonId)
def AddStageEnterEchelon01Starttile(builder, StageEnterEchelon01Starttile): builder.PrependInt64Slot(2, StageEnterEchelon01Starttile, 0)
def FixedStrategyExcelAddStageEnterEchelon01Starttile(builder, StageEnterEchelon01Starttile):
    """This method is deprecated. Please switch to AddStageEnterEchelon01Starttile."""
    return AddStageEnterEchelon01Starttile(builder, StageEnterEchelon01Starttile)
def AddStageEnterEchelon02FixedEchelonId(builder, StageEnterEchelon02FixedEchelonId): builder.PrependInt64Slot(3, StageEnterEchelon02FixedEchelonId, 0)
def FixedStrategyExcelAddStageEnterEchelon02FixedEchelonId(builder, StageEnterEchelon02FixedEchelonId):
    """This method is deprecated. Please switch to AddStageEnterEchelon02FixedEchelonId."""
    return AddStageEnterEchelon02FixedEchelonId(builder, StageEnterEchelon02FixedEchelonId)
def AddStageEnterEchelon02Starttile(builder, StageEnterEchelon02Starttile): builder.PrependInt64Slot(4, StageEnterEchelon02Starttile, 0)
def FixedStrategyExcelAddStageEnterEchelon02Starttile(builder, StageEnterEchelon02Starttile):
    """This method is deprecated. Please switch to AddStageEnterEchelon02Starttile."""
    return AddStageEnterEchelon02Starttile(builder, StageEnterEchelon02Starttile)
def AddStageEnterEchelon03FixedEchelonId(builder, StageEnterEchelon03FixedEchelonId): builder.PrependInt64Slot(5, StageEnterEchelon03FixedEchelonId, 0)
def FixedStrategyExcelAddStageEnterEchelon03FixedEchelonId(builder, StageEnterEchelon03FixedEchelonId):
    """This method is deprecated. Please switch to AddStageEnterEchelon03FixedEchelonId."""
    return AddStageEnterEchelon03FixedEchelonId(builder, StageEnterEchelon03FixedEchelonId)
def AddStageEnterEchelon03Starttile(builder, StageEnterEchelon03Starttile): builder.PrependInt64Slot(6, StageEnterEchelon03Starttile, 0)
def FixedStrategyExcelAddStageEnterEchelon03Starttile(builder, StageEnterEchelon03Starttile):
    """This method is deprecated. Please switch to AddStageEnterEchelon03Starttile."""
    return AddStageEnterEchelon03Starttile(builder, StageEnterEchelon03Starttile)
def AddStageEnterEchelon04FixedEchelonId(builder, StageEnterEchelon04FixedEchelonId): builder.PrependInt64Slot(7, StageEnterEchelon04FixedEchelonId, 0)
def FixedStrategyExcelAddStageEnterEchelon04FixedEchelonId(builder, StageEnterEchelon04FixedEchelonId):
    """This method is deprecated. Please switch to AddStageEnterEchelon04FixedEchelonId."""
    return AddStageEnterEchelon04FixedEchelonId(builder, StageEnterEchelon04FixedEchelonId)
def AddStageEnterEchelon04Starttile(builder, StageEnterEchelon04Starttile): builder.PrependInt64Slot(8, StageEnterEchelon04Starttile, 0)
def FixedStrategyExcelAddStageEnterEchelon04Starttile(builder, StageEnterEchelon04Starttile):
    """This method is deprecated. Please switch to AddStageEnterEchelon04Starttile."""
    return AddStageEnterEchelon04Starttile(builder, StageEnterEchelon04Starttile)
def End(builder): return builder.EndObject()
def FixedStrategyExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)