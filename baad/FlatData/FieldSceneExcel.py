# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FieldSceneExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FieldSceneExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFieldSceneExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FieldSceneExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FieldSceneExcel
    def UniqueId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSceneExcel
    def DateId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSceneExcel
    def GroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSceneExcel
    def ArtLevelPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldSceneExcel
    def DesignLevelPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldSceneExcel
    def BGMId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSceneExcel
    def ConditionalBGMQuestId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldSceneExcel
    def ConditionalBGMQuestIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldSceneExcel
    def ConditionalBGMQuestIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldSceneExcel
    def ConditionalBGMQuestIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        return o == 0

    # FieldSceneExcel
    def BeginConditionalBGMScenarioGroupId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldSceneExcel
    def BeginConditionalBGMScenarioGroupIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldSceneExcel
    def BeginConditionalBGMScenarioGroupIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldSceneExcel
    def BeginConditionalBGMScenarioGroupIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        return o == 0

    # FieldSceneExcel
    def EndConditionalBGMScenarioGroupId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldSceneExcel
    def EndConditionalBGMScenarioGroupIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldSceneExcel
    def EndConditionalBGMScenarioGroupIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldSceneExcel
    def EndConditionalBGMScenarioGroupIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        return o == 0

    # FieldSceneExcel
    def ConditionalBGMId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # FieldSceneExcel
    def ConditionalBGMIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # FieldSceneExcel
    def ConditionalBGMIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # FieldSceneExcel
    def ConditionalBGMIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        return o == 0

def Start(builder): builder.StartObject(10)
def FieldSceneExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddUniqueId(builder, UniqueId): builder.PrependInt64Slot(0, UniqueId, 0)
def FieldSceneExcelAddUniqueId(builder, UniqueId):
    """This method is deprecated. Please switch to AddUniqueId."""
    return AddUniqueId(builder, UniqueId)
def AddDateId(builder, DateId): builder.PrependInt64Slot(1, DateId, 0)
def FieldSceneExcelAddDateId(builder, DateId):
    """This method is deprecated. Please switch to AddDateId."""
    return AddDateId(builder, DateId)
def AddGroupId(builder, GroupId): builder.PrependInt64Slot(2, GroupId, 0)
def FieldSceneExcelAddGroupId(builder, GroupId):
    """This method is deprecated. Please switch to AddGroupId."""
    return AddGroupId(builder, GroupId)
def AddArtLevelPath(builder, ArtLevelPath): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(ArtLevelPath), 0)
def FieldSceneExcelAddArtLevelPath(builder, ArtLevelPath):
    """This method is deprecated. Please switch to AddArtLevelPath."""
    return AddArtLevelPath(builder, ArtLevelPath)
def AddDesignLevelPath(builder, DesignLevelPath): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(DesignLevelPath), 0)
def FieldSceneExcelAddDesignLevelPath(builder, DesignLevelPath):
    """This method is deprecated. Please switch to AddDesignLevelPath."""
    return AddDesignLevelPath(builder, DesignLevelPath)
def AddBGMId(builder, BGMId): builder.PrependInt64Slot(5, BGMId, 0)
def FieldSceneExcelAddBGMId(builder, BGMId):
    """This method is deprecated. Please switch to AddBGMId."""
    return AddBGMId(builder, BGMId)
def AddConditionalBGMQuestId(builder, ConditionalBGMQuestId): builder.PrependUOffsetTRelativeSlot(6, flatbuffers.number_types.UOffsetTFlags.py_type(ConditionalBGMQuestId), 0)
def FieldSceneExcelAddConditionalBGMQuestId(builder, ConditionalBGMQuestId):
    """This method is deprecated. Please switch to AddConditionalBGMQuestId."""
    return AddConditionalBGMQuestId(builder, ConditionalBGMQuestId)
def StartConditionalBGMQuestIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldSceneExcelStartConditionalBGMQuestIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartConditionalBGMQuestIdVector(builder, numElems)
def AddBeginConditionalBGMScenarioGroupId(builder, BeginConditionalBGMScenarioGroupId): builder.PrependUOffsetTRelativeSlot(7, flatbuffers.number_types.UOffsetTFlags.py_type(BeginConditionalBGMScenarioGroupId), 0)
def FieldSceneExcelAddBeginConditionalBGMScenarioGroupId(builder, BeginConditionalBGMScenarioGroupId):
    """This method is deprecated. Please switch to AddBeginConditionalBGMScenarioGroupId."""
    return AddBeginConditionalBGMScenarioGroupId(builder, BeginConditionalBGMScenarioGroupId)
def StartBeginConditionalBGMScenarioGroupIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldSceneExcelStartBeginConditionalBGMScenarioGroupIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBeginConditionalBGMScenarioGroupIdVector(builder, numElems)
def AddEndConditionalBGMScenarioGroupId(builder, EndConditionalBGMScenarioGroupId): builder.PrependUOffsetTRelativeSlot(8, flatbuffers.number_types.UOffsetTFlags.py_type(EndConditionalBGMScenarioGroupId), 0)
def FieldSceneExcelAddEndConditionalBGMScenarioGroupId(builder, EndConditionalBGMScenarioGroupId):
    """This method is deprecated. Please switch to AddEndConditionalBGMScenarioGroupId."""
    return AddEndConditionalBGMScenarioGroupId(builder, EndConditionalBGMScenarioGroupId)
def StartEndConditionalBGMScenarioGroupIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldSceneExcelStartEndConditionalBGMScenarioGroupIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartEndConditionalBGMScenarioGroupIdVector(builder, numElems)
def AddConditionalBGMId(builder, ConditionalBGMId): builder.PrependUOffsetTRelativeSlot(9, flatbuffers.number_types.UOffsetTFlags.py_type(ConditionalBGMId), 0)
def FieldSceneExcelAddConditionalBGMId(builder, ConditionalBGMId):
    """This method is deprecated. Please switch to AddConditionalBGMId."""
    return AddConditionalBGMId(builder, ConditionalBGMId)
def StartConditionalBGMIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def FieldSceneExcelStartConditionalBGMIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartConditionalBGMIdVector(builder, numElems)
def End(builder): return builder.EndObject()
def FieldSceneExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)