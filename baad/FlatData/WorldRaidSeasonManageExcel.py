# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class WorldRaidSeasonManageExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = WorldRaidSeasonManageExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsWorldRaidSeasonManageExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # WorldRaidSeasonManageExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # WorldRaidSeasonManageExcel
    def SeasonId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # WorldRaidSeasonManageExcel
    def EventContentId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # WorldRaidSeasonManageExcel
    def EnterTicket(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # WorldRaidSeasonManageExcel
    def WorldRaidLobbyScene(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # WorldRaidSeasonManageExcel
    def WorldRaidLobbyBanner(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # WorldRaidSeasonManageExcel
    def WorldRaidLobbyBG(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # WorldRaidSeasonManageExcel
    def WorldRaidLobbyBannerShow(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # WorldRaidSeasonManageExcel
    def SeasonOpenCondition(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # WorldRaidSeasonManageExcel
    def WorldRaidLobbyEnterScenario(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # WorldRaidSeasonManageExcel
    def CanPlayNotSeasonTime(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # WorldRaidSeasonManageExcel
    def WorldRaidUniqueThemeLobbyUI(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # WorldRaidSeasonManageExcel
    def WorldRaidUniqueThemeName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # WorldRaidSeasonManageExcel
    def CanWorldRaidGemEnter(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # WorldRaidSeasonManageExcel
    def HideWorldRaidTicketUI(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # WorldRaidSeasonManageExcel
    def UseWorldRaidCommonToast(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # WorldRaidSeasonManageExcel
    def OpenRaidBossGroupId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # WorldRaidSeasonManageExcel
    def OpenRaidBossGroupIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # WorldRaidSeasonManageExcel
    def OpenRaidBossGroupIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # WorldRaidSeasonManageExcel
    def OpenRaidBossGroupIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        return o == 0

    # WorldRaidSeasonManageExcel
    def BossSpawnTime(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # WorldRaidSeasonManageExcel
    def BossSpawnTimeLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # WorldRaidSeasonManageExcel
    def BossSpawnTimeIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        return o == 0

    # WorldRaidSeasonManageExcel
    def EliminateTime(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # WorldRaidSeasonManageExcel
    def EliminateTimeLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # WorldRaidSeasonManageExcel
    def EliminateTimeIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        return o == 0

    # WorldRaidSeasonManageExcel
    def ScenarioOutputConditionId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # WorldRaidSeasonManageExcel
    def ScenarioOutputConditionIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # WorldRaidSeasonManageExcel
    def ScenarioOutputConditionIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # WorldRaidSeasonManageExcel
    def ScenarioOutputConditionIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        return o == 0

    # WorldRaidSeasonManageExcel
    def ConditionScenarioGroupid(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # WorldRaidSeasonManageExcel
    def ConditionScenarioGroupidAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # WorldRaidSeasonManageExcel
    def ConditionScenarioGroupidLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # WorldRaidSeasonManageExcel
    def ConditionScenarioGroupidIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        return o == 0

    # WorldRaidSeasonManageExcel
    def WorldRaidMapEnterOperator(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(44))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # WorldRaidSeasonManageExcel
    def UseFavorRankBuff(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(46))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

def Start(builder): builder.StartObject(22)
def WorldRaidSeasonManageExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddSeasonId(builder, SeasonId): builder.PrependInt64Slot(0, SeasonId, 0)
def WorldRaidSeasonManageExcelAddSeasonId(builder, SeasonId):
    """This method is deprecated. Please switch to AddSeasonId."""
    return AddSeasonId(builder, SeasonId)
def AddEventContentId(builder, EventContentId): builder.PrependInt64Slot(1, EventContentId, 0)
def WorldRaidSeasonManageExcelAddEventContentId(builder, EventContentId):
    """This method is deprecated. Please switch to AddEventContentId."""
    return AddEventContentId(builder, EventContentId)
def AddEnterTicket(builder, EnterTicket): builder.PrependInt32Slot(2, EnterTicket, 0)
def WorldRaidSeasonManageExcelAddEnterTicket(builder, EnterTicket):
    """This method is deprecated. Please switch to AddEnterTicket."""
    return AddEnterTicket(builder, EnterTicket)
def AddWorldRaidLobbyScene(builder, WorldRaidLobbyScene): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(WorldRaidLobbyScene), 0)
def WorldRaidSeasonManageExcelAddWorldRaidLobbyScene(builder, WorldRaidLobbyScene):
    """This method is deprecated. Please switch to AddWorldRaidLobbyScene."""
    return AddWorldRaidLobbyScene(builder, WorldRaidLobbyScene)
def AddWorldRaidLobbyBanner(builder, WorldRaidLobbyBanner): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(WorldRaidLobbyBanner), 0)
def WorldRaidSeasonManageExcelAddWorldRaidLobbyBanner(builder, WorldRaidLobbyBanner):
    """This method is deprecated. Please switch to AddWorldRaidLobbyBanner."""
    return AddWorldRaidLobbyBanner(builder, WorldRaidLobbyBanner)
def AddWorldRaidLobbyBG(builder, WorldRaidLobbyBG): builder.PrependUOffsetTRelativeSlot(5, flatbuffers.number_types.UOffsetTFlags.py_type(WorldRaidLobbyBG), 0)
def WorldRaidSeasonManageExcelAddWorldRaidLobbyBG(builder, WorldRaidLobbyBG):
    """This method is deprecated. Please switch to AddWorldRaidLobbyBG."""
    return AddWorldRaidLobbyBG(builder, WorldRaidLobbyBG)
def AddWorldRaidLobbyBannerShow(builder, WorldRaidLobbyBannerShow): builder.PrependBoolSlot(6, WorldRaidLobbyBannerShow, 0)
def WorldRaidSeasonManageExcelAddWorldRaidLobbyBannerShow(builder, WorldRaidLobbyBannerShow):
    """This method is deprecated. Please switch to AddWorldRaidLobbyBannerShow."""
    return AddWorldRaidLobbyBannerShow(builder, WorldRaidLobbyBannerShow)
def AddSeasonOpenCondition(builder, SeasonOpenCondition): builder.PrependInt64Slot(7, SeasonOpenCondition, 0)
def WorldRaidSeasonManageExcelAddSeasonOpenCondition(builder, SeasonOpenCondition):
    """This method is deprecated. Please switch to AddSeasonOpenCondition."""
    return AddSeasonOpenCondition(builder, SeasonOpenCondition)
def AddWorldRaidLobbyEnterScenario(builder, WorldRaidLobbyEnterScenario): builder.PrependInt64Slot(8, WorldRaidLobbyEnterScenario, 0)
def WorldRaidSeasonManageExcelAddWorldRaidLobbyEnterScenario(builder, WorldRaidLobbyEnterScenario):
    """This method is deprecated. Please switch to AddWorldRaidLobbyEnterScenario."""
    return AddWorldRaidLobbyEnterScenario(builder, WorldRaidLobbyEnterScenario)
def AddCanPlayNotSeasonTime(builder, CanPlayNotSeasonTime): builder.PrependBoolSlot(9, CanPlayNotSeasonTime, 0)
def WorldRaidSeasonManageExcelAddCanPlayNotSeasonTime(builder, CanPlayNotSeasonTime):
    """This method is deprecated. Please switch to AddCanPlayNotSeasonTime."""
    return AddCanPlayNotSeasonTime(builder, CanPlayNotSeasonTime)
def AddWorldRaidUniqueThemeLobbyUI(builder, WorldRaidUniqueThemeLobbyUI): builder.PrependBoolSlot(10, WorldRaidUniqueThemeLobbyUI, 0)
def WorldRaidSeasonManageExcelAddWorldRaidUniqueThemeLobbyUI(builder, WorldRaidUniqueThemeLobbyUI):
    """This method is deprecated. Please switch to AddWorldRaidUniqueThemeLobbyUI."""
    return AddWorldRaidUniqueThemeLobbyUI(builder, WorldRaidUniqueThemeLobbyUI)
def AddWorldRaidUniqueThemeName(builder, WorldRaidUniqueThemeName): builder.PrependUOffsetTRelativeSlot(11, flatbuffers.number_types.UOffsetTFlags.py_type(WorldRaidUniqueThemeName), 0)
def WorldRaidSeasonManageExcelAddWorldRaidUniqueThemeName(builder, WorldRaidUniqueThemeName):
    """This method is deprecated. Please switch to AddWorldRaidUniqueThemeName."""
    return AddWorldRaidUniqueThemeName(builder, WorldRaidUniqueThemeName)
def AddCanWorldRaidGemEnter(builder, CanWorldRaidGemEnter): builder.PrependBoolSlot(12, CanWorldRaidGemEnter, 0)
def WorldRaidSeasonManageExcelAddCanWorldRaidGemEnter(builder, CanWorldRaidGemEnter):
    """This method is deprecated. Please switch to AddCanWorldRaidGemEnter."""
    return AddCanWorldRaidGemEnter(builder, CanWorldRaidGemEnter)
def AddHideWorldRaidTicketUI(builder, HideWorldRaidTicketUI): builder.PrependBoolSlot(13, HideWorldRaidTicketUI, 0)
def WorldRaidSeasonManageExcelAddHideWorldRaidTicketUI(builder, HideWorldRaidTicketUI):
    """This method is deprecated. Please switch to AddHideWorldRaidTicketUI."""
    return AddHideWorldRaidTicketUI(builder, HideWorldRaidTicketUI)
def AddUseWorldRaidCommonToast(builder, UseWorldRaidCommonToast): builder.PrependBoolSlot(14, UseWorldRaidCommonToast, 0)
def WorldRaidSeasonManageExcelAddUseWorldRaidCommonToast(builder, UseWorldRaidCommonToast):
    """This method is deprecated. Please switch to AddUseWorldRaidCommonToast."""
    return AddUseWorldRaidCommonToast(builder, UseWorldRaidCommonToast)
def AddOpenRaidBossGroupId(builder, OpenRaidBossGroupId): builder.PrependUOffsetTRelativeSlot(15, flatbuffers.number_types.UOffsetTFlags.py_type(OpenRaidBossGroupId), 0)
def WorldRaidSeasonManageExcelAddOpenRaidBossGroupId(builder, OpenRaidBossGroupId):
    """This method is deprecated. Please switch to AddOpenRaidBossGroupId."""
    return AddOpenRaidBossGroupId(builder, OpenRaidBossGroupId)
def StartOpenRaidBossGroupIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def WorldRaidSeasonManageExcelStartOpenRaidBossGroupIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartOpenRaidBossGroupIdVector(builder, numElems)
def AddBossSpawnTime(builder, BossSpawnTime): builder.PrependUOffsetTRelativeSlot(16, flatbuffers.number_types.UOffsetTFlags.py_type(BossSpawnTime), 0)
def WorldRaidSeasonManageExcelAddBossSpawnTime(builder, BossSpawnTime):
    """This method is deprecated. Please switch to AddBossSpawnTime."""
    return AddBossSpawnTime(builder, BossSpawnTime)
def StartBossSpawnTimeVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def WorldRaidSeasonManageExcelStartBossSpawnTimeVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBossSpawnTimeVector(builder, numElems)
def AddEliminateTime(builder, EliminateTime): builder.PrependUOffsetTRelativeSlot(17, flatbuffers.number_types.UOffsetTFlags.py_type(EliminateTime), 0)
def WorldRaidSeasonManageExcelAddEliminateTime(builder, EliminateTime):
    """This method is deprecated. Please switch to AddEliminateTime."""
    return AddEliminateTime(builder, EliminateTime)
def StartEliminateTimeVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def WorldRaidSeasonManageExcelStartEliminateTimeVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartEliminateTimeVector(builder, numElems)
def AddScenarioOutputConditionId(builder, ScenarioOutputConditionId): builder.PrependUOffsetTRelativeSlot(18, flatbuffers.number_types.UOffsetTFlags.py_type(ScenarioOutputConditionId), 0)
def WorldRaidSeasonManageExcelAddScenarioOutputConditionId(builder, ScenarioOutputConditionId):
    """This method is deprecated. Please switch to AddScenarioOutputConditionId."""
    return AddScenarioOutputConditionId(builder, ScenarioOutputConditionId)
def StartScenarioOutputConditionIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def WorldRaidSeasonManageExcelStartScenarioOutputConditionIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartScenarioOutputConditionIdVector(builder, numElems)
def AddConditionScenarioGroupid(builder, ConditionScenarioGroupid): builder.PrependUOffsetTRelativeSlot(19, flatbuffers.number_types.UOffsetTFlags.py_type(ConditionScenarioGroupid), 0)
def WorldRaidSeasonManageExcelAddConditionScenarioGroupid(builder, ConditionScenarioGroupid):
    """This method is deprecated. Please switch to AddConditionScenarioGroupid."""
    return AddConditionScenarioGroupid(builder, ConditionScenarioGroupid)
def StartConditionScenarioGroupidVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def WorldRaidSeasonManageExcelStartConditionScenarioGroupidVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartConditionScenarioGroupidVector(builder, numElems)
def AddWorldRaidMapEnterOperator(builder, WorldRaidMapEnterOperator): builder.PrependUOffsetTRelativeSlot(20, flatbuffers.number_types.UOffsetTFlags.py_type(WorldRaidMapEnterOperator), 0)
def WorldRaidSeasonManageExcelAddWorldRaidMapEnterOperator(builder, WorldRaidMapEnterOperator):
    """This method is deprecated. Please switch to AddWorldRaidMapEnterOperator."""
    return AddWorldRaidMapEnterOperator(builder, WorldRaidMapEnterOperator)
def AddUseFavorRankBuff(builder, UseFavorRankBuff): builder.PrependBoolSlot(21, UseFavorRankBuff, 0)
def WorldRaidSeasonManageExcelAddUseFavorRankBuff(builder, UseFavorRankBuff):
    """This method is deprecated. Please switch to AddUseFavorRankBuff."""
    return AddUseFavorRankBuff(builder, UseFavorRankBuff)
def End(builder): return builder.EndObject()
def WorldRaidSeasonManageExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)