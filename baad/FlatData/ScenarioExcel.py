# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ScenarioExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ScenarioExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsScenarioExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ScenarioExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ScenarioExcel
    def None_(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # ScenarioExcel
    def None_AsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # ScenarioExcel
    def None_Length(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ScenarioExcel
    def None_IsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        return o == 0

    # ScenarioExcel
    def Idle(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # ScenarioExcel
    def IdleAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # ScenarioExcel
    def IdleLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ScenarioExcel
    def IdleIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        return o == 0

    # ScenarioExcel
    def Cafe(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioExcel
    def Talk(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioExcel
    def Open(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioExcel
    def EnterConver(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioExcel
    def Center(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioExcel
    def Instant(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioExcel
    def Prologue(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(9)
def ScenarioExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddNone_(builder, None_): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(None_), 0)
def ScenarioExcelAddNone_(builder, None_):
    """This method is deprecated. Please switch to AddNone_."""
    return AddNone_(builder, None_)
def StartNone_Vector(builder, numElems): return builder.StartVector(4, numElems, 4)
def ScenarioExcelStartNone_Vector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartNone_Vector(builder, numElems)
def AddIdle(builder, Idle): builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(Idle), 0)
def ScenarioExcelAddIdle(builder, Idle):
    """This method is deprecated. Please switch to AddIdle."""
    return AddIdle(builder, Idle)
def StartIdleVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def ScenarioExcelStartIdleVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartIdleVector(builder, numElems)
def AddCafe(builder, Cafe): builder.PrependInt32Slot(2, Cafe, 0)
def ScenarioExcelAddCafe(builder, Cafe):
    """This method is deprecated. Please switch to AddCafe."""
    return AddCafe(builder, Cafe)
def AddTalk(builder, Talk): builder.PrependInt32Slot(3, Talk, 0)
def ScenarioExcelAddTalk(builder, Talk):
    """This method is deprecated. Please switch to AddTalk."""
    return AddTalk(builder, Talk)
def AddOpen(builder, Open): builder.PrependInt32Slot(4, Open, 0)
def ScenarioExcelAddOpen(builder, Open):
    """This method is deprecated. Please switch to AddOpen."""
    return AddOpen(builder, Open)
def AddEnterConver(builder, EnterConver): builder.PrependInt32Slot(5, EnterConver, 0)
def ScenarioExcelAddEnterConver(builder, EnterConver):
    """This method is deprecated. Please switch to AddEnterConver."""
    return AddEnterConver(builder, EnterConver)
def AddCenter(builder, Center): builder.PrependInt32Slot(6, Center, 0)
def ScenarioExcelAddCenter(builder, Center):
    """This method is deprecated. Please switch to AddCenter."""
    return AddCenter(builder, Center)
def AddInstant(builder, Instant): builder.PrependInt32Slot(7, Instant, 0)
def ScenarioExcelAddInstant(builder, Instant):
    """This method is deprecated. Please switch to AddInstant."""
    return AddInstant(builder, Instant)
def AddPrologue(builder, Prologue): builder.PrependInt32Slot(8, Prologue, 0)
def ScenarioExcelAddPrologue(builder, Prologue):
    """This method is deprecated. Please switch to AddPrologue."""
    return AddPrologue(builder, Prologue)
def End(builder): return builder.EndObject()
def ScenarioExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)