# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FieldMasteryLevelExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FieldMasteryLevelExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFieldMasteryLevelExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FieldMasteryLevelExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FieldMasteryLevelExcel
    def Level(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # FieldMasteryLevelExcel
    def Id(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldMasteryLevelExcel
    def IdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldMasteryLevelExcel
    def IdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldMasteryLevelExcel
    def IdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        return o == 0

    # FieldMasteryLevelExcel
    def Exp(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldMasteryLevelExcel
    def ExpAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldMasteryLevelExcel
    def ExpLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldMasteryLevelExcel
    def ExpIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        return o == 0

    # FieldMasteryLevelExcel
    def TotalExp(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldMasteryLevelExcel
    def TotalExpAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldMasteryLevelExcel
    def TotalExpLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldMasteryLevelExcel
    def TotalExpIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        return o == 0

    # FieldMasteryLevelExcel
    def RewardId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldMasteryLevelExcel
    def RewardIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldMasteryLevelExcel
    def RewardIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldMasteryLevelExcel
    def RewardIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        return o == 0

def Start(builder): builder.StartObject(5)
def FieldMasteryLevelExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddLevel(builder, Level): builder.PrependInt32Slot(0, Level, 0)
def FieldMasteryLevelExcelAddLevel(builder, Level):
    """This method is deprecated. Please switch to AddLevel."""
    return AddLevel(builder, Level)
def AddId(builder, Id): builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(Id), 0)
def FieldMasteryLevelExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def StartIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldMasteryLevelExcelStartIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartIdVector(builder, numElems)
def AddExp(builder, Exp): builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(Exp), 0)
def FieldMasteryLevelExcelAddExp(builder, Exp):
    """This method is deprecated. Please switch to AddExp."""
    return AddExp(builder, Exp)
def StartExpVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldMasteryLevelExcelStartExpVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartExpVector(builder, numElems)
def AddTotalExp(builder, TotalExp): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(TotalExp), 0)
def FieldMasteryLevelExcelAddTotalExp(builder, TotalExp):
    """This method is deprecated. Please switch to AddTotalExp."""
    return AddTotalExp(builder, TotalExp)
def StartTotalExpVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldMasteryLevelExcelStartTotalExpVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartTotalExpVector(builder, numElems)
def AddRewardId(builder, RewardId): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(RewardId), 0)
def FieldMasteryLevelExcelAddRewardId(builder, RewardId):
    """This method is deprecated. Please switch to AddRewardId."""
    return AddRewardId(builder, RewardId)
def StartRewardIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldMasteryLevelExcelStartRewardIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartRewardIdVector(builder, numElems)
def End(builder): return builder.EndObject()
def FieldMasteryLevelExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)