# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class AcademyMessangerExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = AcademyMessangerExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsAcademyMessangerExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # AcademyMessangerExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # AcademyMessangerExcel
    def MessageGroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def MessageCondition(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def ConditionValue(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def PreConditionGroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def PreConditionFavorScheduleId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def FavorScheduleId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def NextGroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def FeedbackTimeMillisec(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def MessageType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # AcademyMessangerExcel
    def ImagePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageKR(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageJP(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageTH(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageTW(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageEN(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageDE(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AcademyMessangerExcel
    def MessageFR(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def Start(builder): builder.StartObject(19)
def AcademyMessangerExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddMessageGroupId(builder, MessageGroupId): builder.PrependInt64Slot(0, MessageGroupId, 0)
def AcademyMessangerExcelAddMessageGroupId(builder, MessageGroupId):
    """This method is deprecated. Please switch to AddMessageGroupId."""
    return AddMessageGroupId(builder, MessageGroupId)
def AddId(builder, Id): builder.PrependInt64Slot(1, Id, 0)
def AcademyMessangerExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddCharacterId(builder, CharacterId): builder.PrependInt64Slot(2, CharacterId, 0)
def AcademyMessangerExcelAddCharacterId(builder, CharacterId):
    """This method is deprecated. Please switch to AddCharacterId."""
    return AddCharacterId(builder, CharacterId)
def AddMessageCondition(builder, MessageCondition): builder.PrependInt32Slot(3, MessageCondition, 0)
def AcademyMessangerExcelAddMessageCondition(builder, MessageCondition):
    """This method is deprecated. Please switch to AddMessageCondition."""
    return AddMessageCondition(builder, MessageCondition)
def AddConditionValue(builder, ConditionValue): builder.PrependInt64Slot(4, ConditionValue, 0)
def AcademyMessangerExcelAddConditionValue(builder, ConditionValue):
    """This method is deprecated. Please switch to AddConditionValue."""
    return AddConditionValue(builder, ConditionValue)
def AddPreConditionGroupId(builder, PreConditionGroupId): builder.PrependInt64Slot(5, PreConditionGroupId, 0)
def AcademyMessangerExcelAddPreConditionGroupId(builder, PreConditionGroupId):
    """This method is deprecated. Please switch to AddPreConditionGroupId."""
    return AddPreConditionGroupId(builder, PreConditionGroupId)
def AddPreConditionFavorScheduleId(builder, PreConditionFavorScheduleId): builder.PrependInt64Slot(6, PreConditionFavorScheduleId, 0)
def AcademyMessangerExcelAddPreConditionFavorScheduleId(builder, PreConditionFavorScheduleId):
    """This method is deprecated. Please switch to AddPreConditionFavorScheduleId."""
    return AddPreConditionFavorScheduleId(builder, PreConditionFavorScheduleId)
def AddFavorScheduleId(builder, FavorScheduleId): builder.PrependInt64Slot(7, FavorScheduleId, 0)
def AcademyMessangerExcelAddFavorScheduleId(builder, FavorScheduleId):
    """This method is deprecated. Please switch to AddFavorScheduleId."""
    return AddFavorScheduleId(builder, FavorScheduleId)
def AddNextGroupId(builder, NextGroupId): builder.PrependInt64Slot(8, NextGroupId, 0)
def AcademyMessangerExcelAddNextGroupId(builder, NextGroupId):
    """This method is deprecated. Please switch to AddNextGroupId."""
    return AddNextGroupId(builder, NextGroupId)
def AddFeedbackTimeMillisec(builder, FeedbackTimeMillisec): builder.PrependInt64Slot(9, FeedbackTimeMillisec, 0)
def AcademyMessangerExcelAddFeedbackTimeMillisec(builder, FeedbackTimeMillisec):
    """This method is deprecated. Please switch to AddFeedbackTimeMillisec."""
    return AddFeedbackTimeMillisec(builder, FeedbackTimeMillisec)
def AddMessageType(builder, MessageType): builder.PrependInt32Slot(10, MessageType, 0)
def AcademyMessangerExcelAddMessageType(builder, MessageType):
    """This method is deprecated. Please switch to AddMessageType."""
    return AddMessageType(builder, MessageType)
def AddImagePath(builder, ImagePath): builder.PrependUOffsetTRelativeSlot(11, flatbuffers.number_types.UOffsetTFlags.py_type(ImagePath), 0)
def AcademyMessangerExcelAddImagePath(builder, ImagePath):
    """This method is deprecated. Please switch to AddImagePath."""
    return AddImagePath(builder, ImagePath)
def AddMessageKR(builder, MessageKR): builder.PrependUOffsetTRelativeSlot(12, flatbuffers.number_types.UOffsetTFlags.py_type(MessageKR), 0)
def AcademyMessangerExcelAddMessageKR(builder, MessageKR):
    """This method is deprecated. Please switch to AddMessageKR."""
    return AddMessageKR(builder, MessageKR)
def AddMessageJP(builder, MessageJP): builder.PrependUOffsetTRelativeSlot(13, flatbuffers.number_types.UOffsetTFlags.py_type(MessageJP), 0)
def AcademyMessangerExcelAddMessageJP(builder, MessageJP):
    """This method is deprecated. Please switch to AddMessageJP."""
    return AddMessageJP(builder, MessageJP)
def AddMessageTH(builder, MessageTH): builder.PrependUOffsetTRelativeSlot(14, flatbuffers.number_types.UOffsetTFlags.py_type(MessageTH), 0)
def AcademyMessangerExcelAddMessageTH(builder, MessageTH):
    """This method is deprecated. Please switch to AddMessageTH."""
    return AddMessageTH(builder, MessageTH)
def AddMessageTW(builder, MessageTW): builder.PrependUOffsetTRelativeSlot(15, flatbuffers.number_types.UOffsetTFlags.py_type(MessageTW), 0)
def AcademyMessangerExcelAddMessageTW(builder, MessageTW):
    """This method is deprecated. Please switch to AddMessageTW."""
    return AddMessageTW(builder, MessageTW)
def AddMessageEN(builder, MessageEN): builder.PrependUOffsetTRelativeSlot(16, flatbuffers.number_types.UOffsetTFlags.py_type(MessageEN), 0)
def AcademyMessangerExcelAddMessageEN(builder, MessageEN):
    """This method is deprecated. Please switch to AddMessageEN."""
    return AddMessageEN(builder, MessageEN)
def AddMessageDE(builder, MessageDE): builder.PrependUOffsetTRelativeSlot(17, flatbuffers.number_types.UOffsetTFlags.py_type(MessageDE), 0)
def AcademyMessangerExcelAddMessageDE(builder, MessageDE):
    """This method is deprecated. Please switch to AddMessageDE."""
    return AddMessageDE(builder, MessageDE)
def AddMessageFR(builder, MessageFR): builder.PrependUOffsetTRelativeSlot(18, flatbuffers.number_types.UOffsetTFlags.py_type(MessageFR), 0)
def AcademyMessangerExcelAddMessageFR(builder, MessageFR):
    """This method is deprecated. Please switch to AddMessageFR."""
    return AddMessageFR(builder, MessageFR)
def End(builder): return builder.EndObject()
def AcademyMessangerExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)