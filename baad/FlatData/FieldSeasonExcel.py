# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FieldSeasonExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FieldSeasonExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFieldSeasonExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FieldSeasonExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FieldSeasonExcel
    def UniqueId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSeasonExcel
    def EventContentId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSeasonExcel
    def EntryDateId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSeasonExcel
    def InstantEntryDateId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSeasonExcel
    def StartDate(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldSeasonExcel
    def EndDate(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldSeasonExcel
    def LobbyBGMChangeStageId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldSeasonExcel
    def CharacterIconPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldSeasonExcel
    def MasteryImagePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def Start(builder): builder.StartObject(9)
def FieldSeasonExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddUniqueId(builder, UniqueId): builder.PrependInt64Slot(0, UniqueId, 0)
def FieldSeasonExcelAddUniqueId(builder, UniqueId):
    """This method is deprecated. Please switch to AddUniqueId."""
    return AddUniqueId(builder, UniqueId)
def AddEventContentId(builder, EventContentId): builder.PrependInt64Slot(1, EventContentId, 0)
def FieldSeasonExcelAddEventContentId(builder, EventContentId):
    """This method is deprecated. Please switch to AddEventContentId."""
    return AddEventContentId(builder, EventContentId)
def AddEntryDateId(builder, EntryDateId): builder.PrependInt64Slot(2, EntryDateId, 0)
def FieldSeasonExcelAddEntryDateId(builder, EntryDateId):
    """This method is deprecated. Please switch to AddEntryDateId."""
    return AddEntryDateId(builder, EntryDateId)
def AddInstantEntryDateId(builder, InstantEntryDateId): builder.PrependInt64Slot(3, InstantEntryDateId, 0)
def FieldSeasonExcelAddInstantEntryDateId(builder, InstantEntryDateId):
    """This method is deprecated. Please switch to AddInstantEntryDateId."""
    return AddInstantEntryDateId(builder, InstantEntryDateId)
def AddStartDate(builder, StartDate): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(StartDate), 0)
def FieldSeasonExcelAddStartDate(builder, StartDate):
    """This method is deprecated. Please switch to AddStartDate."""
    return AddStartDate(builder, StartDate)
def AddEndDate(builder, EndDate): builder.PrependUOffsetTRelativeSlot(5, flatbuffers.number_types.UOffsetTFlags.py_type(EndDate), 0)
def FieldSeasonExcelAddEndDate(builder, EndDate):
    """This method is deprecated. Please switch to AddEndDate."""
    return AddEndDate(builder, EndDate)
def AddLobbyBGMChangeStageId(builder, LobbyBGMChangeStageId): builder.PrependInt64Slot(6, LobbyBGMChangeStageId, 0)
def FieldSeasonExcelAddLobbyBGMChangeStageId(builder, LobbyBGMChangeStageId):
    """This method is deprecated. Please switch to AddLobbyBGMChangeStageId."""
    return AddLobbyBGMChangeStageId(builder, LobbyBGMChangeStageId)
def AddCharacterIconPath(builder, CharacterIconPath): builder.PrependUOffsetTRelativeSlot(7, flatbuffers.number_types.UOffsetTFlags.py_type(CharacterIconPath), 0)
def FieldSeasonExcelAddCharacterIconPath(builder, CharacterIconPath):
    """This method is deprecated. Please switch to AddCharacterIconPath."""
    return AddCharacterIconPath(builder, CharacterIconPath)
def AddMasteryImagePath(builder, MasteryImagePath): builder.PrependUOffsetTRelativeSlot(8, flatbuffers.number_types.UOffsetTFlags.py_type(MasteryImagePath), 0)
def FieldSeasonExcelAddMasteryImagePath(builder, MasteryImagePath):
    """This method is deprecated. Please switch to AddMasteryImagePath."""
    return AddMasteryImagePath(builder, MasteryImagePath)
def End(builder): return builder.EndObject()
def FieldSeasonExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)