# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ScenarioModeExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ScenarioModeExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsScenarioModeExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ScenarioModeExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ScenarioModeExcel
    def ModeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def ModeType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def SubType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def VolumeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def ChapterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def EpisodeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def Hide(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioModeExcel
    def Open(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioModeExcel
    def IsContinue(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioModeExcel
    def EpisodeContinueModeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def FrontScenarioGroupId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # ScenarioModeExcel
    def FrontScenarioGroupIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # ScenarioModeExcel
    def FrontScenarioGroupIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ScenarioModeExcel
    def FrontScenarioGroupIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        return o == 0

    # ScenarioModeExcel
    def StrategyId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def GroundId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def IsDefeatBattle(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioModeExcel
    def BattleDuration(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def BackScenarioGroupId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # ScenarioModeExcel
    def BackScenarioGroupIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # ScenarioModeExcel
    def BackScenarioGroupIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ScenarioModeExcel
    def BackScenarioGroupIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        return o == 0

    # ScenarioModeExcel
    def ClearedModeId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # ScenarioModeExcel
    def ClearedModeIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # ScenarioModeExcel
    def ClearedModeIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ScenarioModeExcel
    def ClearedModeIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        return o == 0

    # ScenarioModeExcel
    def ScenarioModeRewardId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def IsScenarioSpecialReward(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioModeExcel
    def AccountLevelLimit(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def ClearedStageId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(44))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def NeedClub(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(46))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def NeedClubStudentCount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(48))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def EventContentId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(50))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def EventContentType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(52))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def EventContentCondition(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(54))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def EventContentConditionGroup(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(56))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def MapDifficulty(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(58))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def StepIndex(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(60))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def RecommendLevel(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(62))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def EventIconParcelPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(64))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioModeExcel
    def EventBannerTitle(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(66))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def Lof(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(68))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ScenarioModeExcel
    def StageTopography_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(70))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def FixedEchelonId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(72))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def CompleteReportEventName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(74))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioModeExcel
    def EchelonExtensionType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(76))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioModeExcel
    def CollectionGroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(78))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(38)
def ScenarioModeExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddModeId(builder, ModeId): builder.PrependInt64Slot(0, ModeId, 0)
def ScenarioModeExcelAddModeId(builder, ModeId):
    """This method is deprecated. Please switch to AddModeId."""
    return AddModeId(builder, ModeId)
def AddModeType(builder, ModeType): builder.PrependInt32Slot(1, ModeType, 0)
def ScenarioModeExcelAddModeType(builder, ModeType):
    """This method is deprecated. Please switch to AddModeType."""
    return AddModeType(builder, ModeType)
def AddSubType(builder, SubType): builder.PrependInt32Slot(2, SubType, 0)
def ScenarioModeExcelAddSubType(builder, SubType):
    """This method is deprecated. Please switch to AddSubType."""
    return AddSubType(builder, SubType)
def AddVolumeId(builder, VolumeId): builder.PrependInt64Slot(3, VolumeId, 0)
def ScenarioModeExcelAddVolumeId(builder, VolumeId):
    """This method is deprecated. Please switch to AddVolumeId."""
    return AddVolumeId(builder, VolumeId)
def AddChapterId(builder, ChapterId): builder.PrependInt64Slot(4, ChapterId, 0)
def ScenarioModeExcelAddChapterId(builder, ChapterId):
    """This method is deprecated. Please switch to AddChapterId."""
    return AddChapterId(builder, ChapterId)
def AddEpisodeId(builder, EpisodeId): builder.PrependInt64Slot(5, EpisodeId, 0)
def ScenarioModeExcelAddEpisodeId(builder, EpisodeId):
    """This method is deprecated. Please switch to AddEpisodeId."""
    return AddEpisodeId(builder, EpisodeId)
def AddHide(builder, Hide): builder.PrependBoolSlot(6, Hide, 0)
def ScenarioModeExcelAddHide(builder, Hide):
    """This method is deprecated. Please switch to AddHide."""
    return AddHide(builder, Hide)
def AddOpen(builder, Open): builder.PrependBoolSlot(7, Open, 0)
def ScenarioModeExcelAddOpen(builder, Open):
    """This method is deprecated. Please switch to AddOpen."""
    return AddOpen(builder, Open)
def AddIsContinue(builder, IsContinue): builder.PrependBoolSlot(8, IsContinue, 0)
def ScenarioModeExcelAddIsContinue(builder, IsContinue):
    """This method is deprecated. Please switch to AddIsContinue."""
    return AddIsContinue(builder, IsContinue)
def AddEpisodeContinueModeId(builder, EpisodeContinueModeId): builder.PrependInt64Slot(9, EpisodeContinueModeId, 0)
def ScenarioModeExcelAddEpisodeContinueModeId(builder, EpisodeContinueModeId):
    """This method is deprecated. Please switch to AddEpisodeContinueModeId."""
    return AddEpisodeContinueModeId(builder, EpisodeContinueModeId)
def AddFrontScenarioGroupId(builder, FrontScenarioGroupId): builder.PrependUOffsetTRelativeSlot(10, flatbuffers.number_types.UOffsetTFlags.py_type(FrontScenarioGroupId), 0)
def ScenarioModeExcelAddFrontScenarioGroupId(builder, FrontScenarioGroupId):
    """This method is deprecated. Please switch to AddFrontScenarioGroupId."""
    return AddFrontScenarioGroupId(builder, FrontScenarioGroupId)
def StartFrontScenarioGroupIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def ScenarioModeExcelStartFrontScenarioGroupIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartFrontScenarioGroupIdVector(builder, numElems)
def AddStrategyId(builder, StrategyId): builder.PrependInt64Slot(11, StrategyId, 0)
def ScenarioModeExcelAddStrategyId(builder, StrategyId):
    """This method is deprecated. Please switch to AddStrategyId."""
    return AddStrategyId(builder, StrategyId)
def AddGroundId(builder, GroundId): builder.PrependInt64Slot(12, GroundId, 0)
def ScenarioModeExcelAddGroundId(builder, GroundId):
    """This method is deprecated. Please switch to AddGroundId."""
    return AddGroundId(builder, GroundId)
def AddIsDefeatBattle(builder, IsDefeatBattle): builder.PrependBoolSlot(13, IsDefeatBattle, 0)
def ScenarioModeExcelAddIsDefeatBattle(builder, IsDefeatBattle):
    """This method is deprecated. Please switch to AddIsDefeatBattle."""
    return AddIsDefeatBattle(builder, IsDefeatBattle)
def AddBattleDuration(builder, BattleDuration): builder.PrependInt64Slot(14, BattleDuration, 0)
def ScenarioModeExcelAddBattleDuration(builder, BattleDuration):
    """This method is deprecated. Please switch to AddBattleDuration."""
    return AddBattleDuration(builder, BattleDuration)
def AddBackScenarioGroupId(builder, BackScenarioGroupId): builder.PrependUOffsetTRelativeSlot(15, flatbuffers.number_types.UOffsetTFlags.py_type(BackScenarioGroupId), 0)
def ScenarioModeExcelAddBackScenarioGroupId(builder, BackScenarioGroupId):
    """This method is deprecated. Please switch to AddBackScenarioGroupId."""
    return AddBackScenarioGroupId(builder, BackScenarioGroupId)
def StartBackScenarioGroupIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def ScenarioModeExcelStartBackScenarioGroupIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBackScenarioGroupIdVector(builder, numElems)
def AddClearedModeId(builder, ClearedModeId): builder.PrependUOffsetTRelativeSlot(16, flatbuffers.number_types.UOffsetTFlags.py_type(ClearedModeId), 0)
def ScenarioModeExcelAddClearedModeId(builder, ClearedModeId):
    """This method is deprecated. Please switch to AddClearedModeId."""
    return AddClearedModeId(builder, ClearedModeId)
def StartClearedModeIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def ScenarioModeExcelStartClearedModeIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartClearedModeIdVector(builder, numElems)
def AddScenarioModeRewardId(builder, ScenarioModeRewardId): builder.PrependInt64Slot(17, ScenarioModeRewardId, 0)
def ScenarioModeExcelAddScenarioModeRewardId(builder, ScenarioModeRewardId):
    """This method is deprecated. Please switch to AddScenarioModeRewardId."""
    return AddScenarioModeRewardId(builder, ScenarioModeRewardId)
def AddIsScenarioSpecialReward(builder, IsScenarioSpecialReward): builder.PrependBoolSlot(18, IsScenarioSpecialReward, 0)
def ScenarioModeExcelAddIsScenarioSpecialReward(builder, IsScenarioSpecialReward):
    """This method is deprecated. Please switch to AddIsScenarioSpecialReward."""
    return AddIsScenarioSpecialReward(builder, IsScenarioSpecialReward)
def AddAccountLevelLimit(builder, AccountLevelLimit): builder.PrependInt64Slot(19, AccountLevelLimit, 0)
def ScenarioModeExcelAddAccountLevelLimit(builder, AccountLevelLimit):
    """This method is deprecated. Please switch to AddAccountLevelLimit."""
    return AddAccountLevelLimit(builder, AccountLevelLimit)
def AddClearedStageId(builder, ClearedStageId): builder.PrependInt64Slot(20, ClearedStageId, 0)
def ScenarioModeExcelAddClearedStageId(builder, ClearedStageId):
    """This method is deprecated. Please switch to AddClearedStageId."""
    return AddClearedStageId(builder, ClearedStageId)
def AddNeedClub(builder, NeedClub): builder.PrependInt32Slot(21, NeedClub, 0)
def ScenarioModeExcelAddNeedClub(builder, NeedClub):
    """This method is deprecated. Please switch to AddNeedClub."""
    return AddNeedClub(builder, NeedClub)
def AddNeedClubStudentCount(builder, NeedClubStudentCount): builder.PrependInt32Slot(22, NeedClubStudentCount, 0)
def ScenarioModeExcelAddNeedClubStudentCount(builder, NeedClubStudentCount):
    """This method is deprecated. Please switch to AddNeedClubStudentCount."""
    return AddNeedClubStudentCount(builder, NeedClubStudentCount)
def AddEventContentId(builder, EventContentId): builder.PrependInt64Slot(23, EventContentId, 0)
def ScenarioModeExcelAddEventContentId(builder, EventContentId):
    """This method is deprecated. Please switch to AddEventContentId."""
    return AddEventContentId(builder, EventContentId)
def AddEventContentType_(builder, EventContentType_): builder.PrependInt32Slot(24, EventContentType_, 0)
def ScenarioModeExcelAddEventContentType_(builder, EventContentType_):
    """This method is deprecated. Please switch to AddEventContentType_."""
    return AddEventContentType_(builder, EventContentType_)
def AddEventContentCondition(builder, EventContentCondition): builder.PrependInt64Slot(25, EventContentCondition, 0)
def ScenarioModeExcelAddEventContentCondition(builder, EventContentCondition):
    """This method is deprecated. Please switch to AddEventContentCondition."""
    return AddEventContentCondition(builder, EventContentCondition)
def AddEventContentConditionGroup(builder, EventContentConditionGroup): builder.PrependInt64Slot(26, EventContentConditionGroup, 0)
def ScenarioModeExcelAddEventContentConditionGroup(builder, EventContentConditionGroup):
    """This method is deprecated. Please switch to AddEventContentConditionGroup."""
    return AddEventContentConditionGroup(builder, EventContentConditionGroup)
def AddMapDifficulty(builder, MapDifficulty): builder.PrependInt32Slot(27, MapDifficulty, 0)
def ScenarioModeExcelAddMapDifficulty(builder, MapDifficulty):
    """This method is deprecated. Please switch to AddMapDifficulty."""
    return AddMapDifficulty(builder, MapDifficulty)
def AddStepIndex(builder, StepIndex): builder.PrependInt32Slot(28, StepIndex, 0)
def ScenarioModeExcelAddStepIndex(builder, StepIndex):
    """This method is deprecated. Please switch to AddStepIndex."""
    return AddStepIndex(builder, StepIndex)
def AddRecommendLevel(builder, RecommendLevel): builder.PrependInt32Slot(29, RecommendLevel, 0)
def ScenarioModeExcelAddRecommendLevel(builder, RecommendLevel):
    """This method is deprecated. Please switch to AddRecommendLevel."""
    return AddRecommendLevel(builder, RecommendLevel)
def AddEventIconParcelPath(builder, EventIconParcelPath): builder.PrependUOffsetTRelativeSlot(30, flatbuffers.number_types.UOffsetTFlags.py_type(EventIconParcelPath), 0)
def ScenarioModeExcelAddEventIconParcelPath(builder, EventIconParcelPath):
    """This method is deprecated. Please switch to AddEventIconParcelPath."""
    return AddEventIconParcelPath(builder, EventIconParcelPath)
def AddEventBannerTitle(builder, EventBannerTitle): builder.PrependUint32Slot(31, EventBannerTitle, 0)
def ScenarioModeExcelAddEventBannerTitle(builder, EventBannerTitle):
    """This method is deprecated. Please switch to AddEventBannerTitle."""
    return AddEventBannerTitle(builder, EventBannerTitle)
def AddLof(builder, Lof): builder.PrependBoolSlot(32, Lof, 0)
def ScenarioModeExcelAddLof(builder, Lof):
    """This method is deprecated. Please switch to AddLof."""
    return AddLof(builder, Lof)
def AddStageTopography_(builder, StageTopography_): builder.PrependInt32Slot(33, StageTopography_, 0)
def ScenarioModeExcelAddStageTopography_(builder, StageTopography_):
    """This method is deprecated. Please switch to AddStageTopography_."""
    return AddStageTopography_(builder, StageTopography_)
def AddFixedEchelonId(builder, FixedEchelonId): builder.PrependInt64Slot(34, FixedEchelonId, 0)
def ScenarioModeExcelAddFixedEchelonId(builder, FixedEchelonId):
    """This method is deprecated. Please switch to AddFixedEchelonId."""
    return AddFixedEchelonId(builder, FixedEchelonId)
def AddCompleteReportEventName(builder, CompleteReportEventName): builder.PrependUOffsetTRelativeSlot(35, flatbuffers.number_types.UOffsetTFlags.py_type(CompleteReportEventName), 0)
def ScenarioModeExcelAddCompleteReportEventName(builder, CompleteReportEventName):
    """This method is deprecated. Please switch to AddCompleteReportEventName."""
    return AddCompleteReportEventName(builder, CompleteReportEventName)
def AddEchelonExtensionType_(builder, EchelonExtensionType_): builder.PrependInt32Slot(36, EchelonExtensionType_, 0)
def ScenarioModeExcelAddEchelonExtensionType_(builder, EchelonExtensionType_):
    """This method is deprecated. Please switch to AddEchelonExtensionType_."""
    return AddEchelonExtensionType_(builder, EchelonExtensionType_)
def AddCollectionGroupId(builder, CollectionGroupId): builder.PrependInt64Slot(37, CollectionGroupId, 0)
def ScenarioModeExcelAddCollectionGroupId(builder, CollectionGroupId):
    """This method is deprecated. Please switch to AddCollectionGroupId."""
    return AddCollectionGroupId(builder, CollectionGroupId)
def End(builder): return builder.EndObject()
def ScenarioModeExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)