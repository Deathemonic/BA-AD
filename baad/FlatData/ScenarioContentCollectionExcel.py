# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ScenarioContentCollectionExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ScenarioContentCollectionExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsScenarioContentCollectionExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ScenarioContentCollectionExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ScenarioContentCollectionExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioContentCollectionExcel
    def GroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioContentCollectionExcel
    def UnlockConditionType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioContentCollectionExcel
    def UnlockConditionParameter(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # ScenarioContentCollectionExcel
    def UnlockConditionParameterAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # ScenarioContentCollectionExcel
    def UnlockConditionParameterLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ScenarioContentCollectionExcel
    def UnlockConditionParameterIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        return o == 0

    # ScenarioContentCollectionExcel
    def MultipleConditionCheckType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioContentCollectionExcel
    def UnlockConditionCount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioContentCollectionExcel
    def IsObject(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioContentCollectionExcel
    def IsHorizon(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioContentCollectionExcel
    def EmblemResource(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioContentCollectionExcel
    def ThumbResource(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioContentCollectionExcel
    def FullResource(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioContentCollectionExcel
    def LocalizeEtcId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # ScenarioContentCollectionExcel
    def SubNameLocalizeCodeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def Start(builder): builder.StartObject(13)
def ScenarioContentCollectionExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def ScenarioContentCollectionExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddGroupId(builder, GroupId): builder.PrependInt64Slot(1, GroupId, 0)
def ScenarioContentCollectionExcelAddGroupId(builder, GroupId):
    """This method is deprecated. Please switch to AddGroupId."""
    return AddGroupId(builder, GroupId)
def AddUnlockConditionType(builder, UnlockConditionType): builder.PrependInt32Slot(2, UnlockConditionType, 0)
def ScenarioContentCollectionExcelAddUnlockConditionType(builder, UnlockConditionType):
    """This method is deprecated. Please switch to AddUnlockConditionType."""
    return AddUnlockConditionType(builder, UnlockConditionType)
def AddUnlockConditionParameter(builder, UnlockConditionParameter): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(UnlockConditionParameter), 0)
def ScenarioContentCollectionExcelAddUnlockConditionParameter(builder, UnlockConditionParameter):
    """This method is deprecated. Please switch to AddUnlockConditionParameter."""
    return AddUnlockConditionParameter(builder, UnlockConditionParameter)
def StartUnlockConditionParameterVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def ScenarioContentCollectionExcelStartUnlockConditionParameterVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartUnlockConditionParameterVector(builder, numElems)
def AddMultipleConditionCheckType_(builder, MultipleConditionCheckType_): builder.PrependInt32Slot(4, MultipleConditionCheckType_, 0)
def ScenarioContentCollectionExcelAddMultipleConditionCheckType_(builder, MultipleConditionCheckType_):
    """This method is deprecated. Please switch to AddMultipleConditionCheckType_."""
    return AddMultipleConditionCheckType_(builder, MultipleConditionCheckType_)
def AddUnlockConditionCount(builder, UnlockConditionCount): builder.PrependInt64Slot(5, UnlockConditionCount, 0)
def ScenarioContentCollectionExcelAddUnlockConditionCount(builder, UnlockConditionCount):
    """This method is deprecated. Please switch to AddUnlockConditionCount."""
    return AddUnlockConditionCount(builder, UnlockConditionCount)
def AddIsObject(builder, IsObject): builder.PrependBoolSlot(6, IsObject, 0)
def ScenarioContentCollectionExcelAddIsObject(builder, IsObject):
    """This method is deprecated. Please switch to AddIsObject."""
    return AddIsObject(builder, IsObject)
def AddIsHorizon(builder, IsHorizon): builder.PrependBoolSlot(7, IsHorizon, 0)
def ScenarioContentCollectionExcelAddIsHorizon(builder, IsHorizon):
    """This method is deprecated. Please switch to AddIsHorizon."""
    return AddIsHorizon(builder, IsHorizon)
def AddEmblemResource(builder, EmblemResource): builder.PrependUOffsetTRelativeSlot(8, flatbuffers.number_types.UOffsetTFlags.py_type(EmblemResource), 0)
def ScenarioContentCollectionExcelAddEmblemResource(builder, EmblemResource):
    """This method is deprecated. Please switch to AddEmblemResource."""
    return AddEmblemResource(builder, EmblemResource)
def AddThumbResource(builder, ThumbResource): builder.PrependUOffsetTRelativeSlot(9, flatbuffers.number_types.UOffsetTFlags.py_type(ThumbResource), 0)
def ScenarioContentCollectionExcelAddThumbResource(builder, ThumbResource):
    """This method is deprecated. Please switch to AddThumbResource."""
    return AddThumbResource(builder, ThumbResource)
def AddFullResource(builder, FullResource): builder.PrependUOffsetTRelativeSlot(10, flatbuffers.number_types.UOffsetTFlags.py_type(FullResource), 0)
def ScenarioContentCollectionExcelAddFullResource(builder, FullResource):
    """This method is deprecated. Please switch to AddFullResource."""
    return AddFullResource(builder, FullResource)
def AddLocalizeEtcId(builder, LocalizeEtcId): builder.PrependUint32Slot(11, LocalizeEtcId, 0)
def ScenarioContentCollectionExcelAddLocalizeEtcId(builder, LocalizeEtcId):
    """This method is deprecated. Please switch to AddLocalizeEtcId."""
    return AddLocalizeEtcId(builder, LocalizeEtcId)
def AddSubNameLocalizeCodeId(builder, SubNameLocalizeCodeId): builder.PrependUOffsetTRelativeSlot(12, flatbuffers.number_types.UOffsetTFlags.py_type(SubNameLocalizeCodeId), 0)
def ScenarioContentCollectionExcelAddSubNameLocalizeCodeId(builder, SubNameLocalizeCodeId):
    """This method is deprecated. Please switch to AddSubNameLocalizeCodeId."""
    return AddSubNameLocalizeCodeId(builder, SubNameLocalizeCodeId)
def End(builder): return builder.EndObject()
def ScenarioContentCollectionExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)