# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class EliminateRaidStageExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = EliminateRaidStageExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsEliminateRaidStageExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # EliminateRaidStageExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # EliminateRaidStageExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def UseBossIndex(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # EliminateRaidStageExcel
    def UseBossAIPhaseSync(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # EliminateRaidStageExcel
    def RaidBossGroup(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def RaidEnterCostType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def RaidEnterCostId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def RaidEnterCostAmount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def BossSpinePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def PortraitPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def BGPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def RaidCharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def BossCharacterId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int64Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 8))
        return 0

    # EliminateRaidStageExcel
    def BossCharacterIdAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int64Flags, o)
        return 0

    # EliminateRaidStageExcel
    def BossCharacterIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # EliminateRaidStageExcel
    def BossCharacterIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        return o == 0

    # EliminateRaidStageExcel
    def Difficulty_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def IsOpen(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # EliminateRaidStageExcel
    def MaxPlayerCount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def RaidRoomLifeTime(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def BattleDuration(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def GroundId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def GroundDevName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def EnterTimeLine(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def TacticEnvironment_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(44))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def DefaultClearScore(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(46))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def MaximumScore(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(48))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def PerSecondMinusScore(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(50))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def HPPercentScore(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(52))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def MinimumAcquisitionScore(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(54))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def MaximumAcquisitionScore(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(56))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def RaidRewardGroupId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(58))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePath(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(60))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # EliminateRaidStageExcel
    def BattleReadyTimelinePathLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(60))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePathIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(60))
        return o == 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseStart(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(62))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseStartAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(62))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseStartLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(62))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseStartIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(62))
        return o == 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseEnd(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(64))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseEndAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(64))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseEndLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(64))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # EliminateRaidStageExcel
    def BattleReadyTimelinePhaseEndIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(64))
        return o == 0

    # EliminateRaidStageExcel
    def VictoryTimelinePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(66))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def PhaseChangeTimelinePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(68))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # EliminateRaidStageExcel
    def TimeLinePhase(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(70))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def EnterScenarioKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(72))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def ClearScenarioKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(74))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def ShowSkillCard(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(76))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # EliminateRaidStageExcel
    def BossBGInfoKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(78))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # EliminateRaidStageExcel
    def EchelonExtensionType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(80))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(39)
def EliminateRaidStageExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def EliminateRaidStageExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddUseBossIndex(builder, UseBossIndex): builder.PrependBoolSlot(1, UseBossIndex, 0)
def EliminateRaidStageExcelAddUseBossIndex(builder, UseBossIndex):
    """This method is deprecated. Please switch to AddUseBossIndex."""
    return AddUseBossIndex(builder, UseBossIndex)
def AddUseBossAIPhaseSync(builder, UseBossAIPhaseSync): builder.PrependBoolSlot(2, UseBossAIPhaseSync, 0)
def EliminateRaidStageExcelAddUseBossAIPhaseSync(builder, UseBossAIPhaseSync):
    """This method is deprecated. Please switch to AddUseBossAIPhaseSync."""
    return AddUseBossAIPhaseSync(builder, UseBossAIPhaseSync)
def AddRaidBossGroup(builder, RaidBossGroup): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(RaidBossGroup), 0)
def EliminateRaidStageExcelAddRaidBossGroup(builder, RaidBossGroup):
    """This method is deprecated. Please switch to AddRaidBossGroup."""
    return AddRaidBossGroup(builder, RaidBossGroup)
def AddRaidEnterCostType(builder, RaidEnterCostType): builder.PrependInt32Slot(4, RaidEnterCostType, 0)
def EliminateRaidStageExcelAddRaidEnterCostType(builder, RaidEnterCostType):
    """This method is deprecated. Please switch to AddRaidEnterCostType."""
    return AddRaidEnterCostType(builder, RaidEnterCostType)
def AddRaidEnterCostId(builder, RaidEnterCostId): builder.PrependInt64Slot(5, RaidEnterCostId, 0)
def EliminateRaidStageExcelAddRaidEnterCostId(builder, RaidEnterCostId):
    """This method is deprecated. Please switch to AddRaidEnterCostId."""
    return AddRaidEnterCostId(builder, RaidEnterCostId)
def AddRaidEnterCostAmount(builder, RaidEnterCostAmount): builder.PrependInt32Slot(6, RaidEnterCostAmount, 0)
def EliminateRaidStageExcelAddRaidEnterCostAmount(builder, RaidEnterCostAmount):
    """This method is deprecated. Please switch to AddRaidEnterCostAmount."""
    return AddRaidEnterCostAmount(builder, RaidEnterCostAmount)
def AddBossSpinePath(builder, BossSpinePath): builder.PrependUOffsetTRelativeSlot(7, flatbuffers.number_types.UOffsetTFlags.py_type(BossSpinePath), 0)
def EliminateRaidStageExcelAddBossSpinePath(builder, BossSpinePath):
    """This method is deprecated. Please switch to AddBossSpinePath."""
    return AddBossSpinePath(builder, BossSpinePath)
def AddPortraitPath(builder, PortraitPath): builder.PrependUOffsetTRelativeSlot(8, flatbuffers.number_types.UOffsetTFlags.py_type(PortraitPath), 0)
def EliminateRaidStageExcelAddPortraitPath(builder, PortraitPath):
    """This method is deprecated. Please switch to AddPortraitPath."""
    return AddPortraitPath(builder, PortraitPath)
def AddBGPath(builder, BGPath): builder.PrependUOffsetTRelativeSlot(9, flatbuffers.number_types.UOffsetTFlags.py_type(BGPath), 0)
def EliminateRaidStageExcelAddBGPath(builder, BGPath):
    """This method is deprecated. Please switch to AddBGPath."""
    return AddBGPath(builder, BGPath)
def AddRaidCharacterId(builder, RaidCharacterId): builder.PrependInt64Slot(10, RaidCharacterId, 0)
def EliminateRaidStageExcelAddRaidCharacterId(builder, RaidCharacterId):
    """This method is deprecated. Please switch to AddRaidCharacterId."""
    return AddRaidCharacterId(builder, RaidCharacterId)
def AddBossCharacterId(builder, BossCharacterId): builder.PrependUOffsetTRelativeSlot(11, flatbuffers.number_types.UOffsetTFlags.py_type(BossCharacterId), 0)
def EliminateRaidStageExcelAddBossCharacterId(builder, BossCharacterId):
    """This method is deprecated. Please switch to AddBossCharacterId."""
    return AddBossCharacterId(builder, BossCharacterId)
def StartBossCharacterIdVector(builder, numElems): return builder.StartVector(8, numElems, 8)
def EliminateRaidStageExcelStartBossCharacterIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBossCharacterIdVector(builder, numElems)
def AddDifficulty_(builder, Difficulty_): builder.PrependInt32Slot(12, Difficulty_, 0)
def EliminateRaidStageExcelAddDifficulty_(builder, Difficulty_):
    """This method is deprecated. Please switch to AddDifficulty_."""
    return AddDifficulty_(builder, Difficulty_)
def AddIsOpen(builder, IsOpen): builder.PrependBoolSlot(13, IsOpen, 0)
def EliminateRaidStageExcelAddIsOpen(builder, IsOpen):
    """This method is deprecated. Please switch to AddIsOpen."""
    return AddIsOpen(builder, IsOpen)
def AddMaxPlayerCount(builder, MaxPlayerCount): builder.PrependInt64Slot(14, MaxPlayerCount, 0)
def EliminateRaidStageExcelAddMaxPlayerCount(builder, MaxPlayerCount):
    """This method is deprecated. Please switch to AddMaxPlayerCount."""
    return AddMaxPlayerCount(builder, MaxPlayerCount)
def AddRaidRoomLifeTime(builder, RaidRoomLifeTime): builder.PrependInt32Slot(15, RaidRoomLifeTime, 0)
def EliminateRaidStageExcelAddRaidRoomLifeTime(builder, RaidRoomLifeTime):
    """This method is deprecated. Please switch to AddRaidRoomLifeTime."""
    return AddRaidRoomLifeTime(builder, RaidRoomLifeTime)
def AddBattleDuration(builder, BattleDuration): builder.PrependInt64Slot(16, BattleDuration, 0)
def EliminateRaidStageExcelAddBattleDuration(builder, BattleDuration):
    """This method is deprecated. Please switch to AddBattleDuration."""
    return AddBattleDuration(builder, BattleDuration)
def AddGroundId(builder, GroundId): builder.PrependInt64Slot(17, GroundId, 0)
def EliminateRaidStageExcelAddGroundId(builder, GroundId):
    """This method is deprecated. Please switch to AddGroundId."""
    return AddGroundId(builder, GroundId)
def AddGroundDevName(builder, GroundDevName): builder.PrependUOffsetTRelativeSlot(18, flatbuffers.number_types.UOffsetTFlags.py_type(GroundDevName), 0)
def EliminateRaidStageExcelAddGroundDevName(builder, GroundDevName):
    """This method is deprecated. Please switch to AddGroundDevName."""
    return AddGroundDevName(builder, GroundDevName)
def AddEnterTimeLine(builder, EnterTimeLine): builder.PrependUOffsetTRelativeSlot(19, flatbuffers.number_types.UOffsetTFlags.py_type(EnterTimeLine), 0)
def EliminateRaidStageExcelAddEnterTimeLine(builder, EnterTimeLine):
    """This method is deprecated. Please switch to AddEnterTimeLine."""
    return AddEnterTimeLine(builder, EnterTimeLine)
def AddTacticEnvironment_(builder, TacticEnvironment_): builder.PrependInt32Slot(20, TacticEnvironment_, 0)
def EliminateRaidStageExcelAddTacticEnvironment_(builder, TacticEnvironment_):
    """This method is deprecated. Please switch to AddTacticEnvironment_."""
    return AddTacticEnvironment_(builder, TacticEnvironment_)
def AddDefaultClearScore(builder, DefaultClearScore): builder.PrependInt64Slot(21, DefaultClearScore, 0)
def EliminateRaidStageExcelAddDefaultClearScore(builder, DefaultClearScore):
    """This method is deprecated. Please switch to AddDefaultClearScore."""
    return AddDefaultClearScore(builder, DefaultClearScore)
def AddMaximumScore(builder, MaximumScore): builder.PrependInt64Slot(22, MaximumScore, 0)
def EliminateRaidStageExcelAddMaximumScore(builder, MaximumScore):
    """This method is deprecated. Please switch to AddMaximumScore."""
    return AddMaximumScore(builder, MaximumScore)
def AddPerSecondMinusScore(builder, PerSecondMinusScore): builder.PrependInt64Slot(23, PerSecondMinusScore, 0)
def EliminateRaidStageExcelAddPerSecondMinusScore(builder, PerSecondMinusScore):
    """This method is deprecated. Please switch to AddPerSecondMinusScore."""
    return AddPerSecondMinusScore(builder, PerSecondMinusScore)
def AddHPPercentScore(builder, HPPercentScore): builder.PrependInt64Slot(24, HPPercentScore, 0)
def EliminateRaidStageExcelAddHPPercentScore(builder, HPPercentScore):
    """This method is deprecated. Please switch to AddHPPercentScore."""
    return AddHPPercentScore(builder, HPPercentScore)
def AddMinimumAcquisitionScore(builder, MinimumAcquisitionScore): builder.PrependInt64Slot(25, MinimumAcquisitionScore, 0)
def EliminateRaidStageExcelAddMinimumAcquisitionScore(builder, MinimumAcquisitionScore):
    """This method is deprecated. Please switch to AddMinimumAcquisitionScore."""
    return AddMinimumAcquisitionScore(builder, MinimumAcquisitionScore)
def AddMaximumAcquisitionScore(builder, MaximumAcquisitionScore): builder.PrependInt64Slot(26, MaximumAcquisitionScore, 0)
def EliminateRaidStageExcelAddMaximumAcquisitionScore(builder, MaximumAcquisitionScore):
    """This method is deprecated. Please switch to AddMaximumAcquisitionScore."""
    return AddMaximumAcquisitionScore(builder, MaximumAcquisitionScore)
def AddRaidRewardGroupId(builder, RaidRewardGroupId): builder.PrependInt64Slot(27, RaidRewardGroupId, 0)
def EliminateRaidStageExcelAddRaidRewardGroupId(builder, RaidRewardGroupId):
    """This method is deprecated. Please switch to AddRaidRewardGroupId."""
    return AddRaidRewardGroupId(builder, RaidRewardGroupId)
def AddBattleReadyTimelinePath(builder, BattleReadyTimelinePath): builder.PrependUOffsetTRelativeSlot(28, flatbuffers.number_types.UOffsetTFlags.py_type(BattleReadyTimelinePath), 0)
def EliminateRaidStageExcelAddBattleReadyTimelinePath(builder, BattleReadyTimelinePath):
    """This method is deprecated. Please switch to AddBattleReadyTimelinePath."""
    return AddBattleReadyTimelinePath(builder, BattleReadyTimelinePath)
def StartBattleReadyTimelinePathVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def EliminateRaidStageExcelStartBattleReadyTimelinePathVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBattleReadyTimelinePathVector(builder, numElems)
def AddBattleReadyTimelinePhaseStart(builder, BattleReadyTimelinePhaseStart): builder.PrependUOffsetTRelativeSlot(29, flatbuffers.number_types.UOffsetTFlags.py_type(BattleReadyTimelinePhaseStart), 0)
def EliminateRaidStageExcelAddBattleReadyTimelinePhaseStart(builder, BattleReadyTimelinePhaseStart):
    """This method is deprecated. Please switch to AddBattleReadyTimelinePhaseStart."""
    return AddBattleReadyTimelinePhaseStart(builder, BattleReadyTimelinePhaseStart)
def StartBattleReadyTimelinePhaseStartVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def EliminateRaidStageExcelStartBattleReadyTimelinePhaseStartVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBattleReadyTimelinePhaseStartVector(builder, numElems)
def AddBattleReadyTimelinePhaseEnd(builder, BattleReadyTimelinePhaseEnd): builder.PrependUOffsetTRelativeSlot(30, flatbuffers.number_types.UOffsetTFlags.py_type(BattleReadyTimelinePhaseEnd), 0)
def EliminateRaidStageExcelAddBattleReadyTimelinePhaseEnd(builder, BattleReadyTimelinePhaseEnd):
    """This method is deprecated. Please switch to AddBattleReadyTimelinePhaseEnd."""
    return AddBattleReadyTimelinePhaseEnd(builder, BattleReadyTimelinePhaseEnd)
def StartBattleReadyTimelinePhaseEndVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def EliminateRaidStageExcelStartBattleReadyTimelinePhaseEndVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartBattleReadyTimelinePhaseEndVector(builder, numElems)
def AddVictoryTimelinePath(builder, VictoryTimelinePath): builder.PrependUOffsetTRelativeSlot(31, flatbuffers.number_types.UOffsetTFlags.py_type(VictoryTimelinePath), 0)
def EliminateRaidStageExcelAddVictoryTimelinePath(builder, VictoryTimelinePath):
    """This method is deprecated. Please switch to AddVictoryTimelinePath."""
    return AddVictoryTimelinePath(builder, VictoryTimelinePath)
def AddPhaseChangeTimelinePath(builder, PhaseChangeTimelinePath): builder.PrependUOffsetTRelativeSlot(32, flatbuffers.number_types.UOffsetTFlags.py_type(PhaseChangeTimelinePath), 0)
def EliminateRaidStageExcelAddPhaseChangeTimelinePath(builder, PhaseChangeTimelinePath):
    """This method is deprecated. Please switch to AddPhaseChangeTimelinePath."""
    return AddPhaseChangeTimelinePath(builder, PhaseChangeTimelinePath)
def AddTimeLinePhase(builder, TimeLinePhase): builder.PrependInt64Slot(33, TimeLinePhase, 0)
def EliminateRaidStageExcelAddTimeLinePhase(builder, TimeLinePhase):
    """This method is deprecated. Please switch to AddTimeLinePhase."""
    return AddTimeLinePhase(builder, TimeLinePhase)
def AddEnterScenarioKey(builder, EnterScenarioKey): builder.PrependUint32Slot(34, EnterScenarioKey, 0)
def EliminateRaidStageExcelAddEnterScenarioKey(builder, EnterScenarioKey):
    """This method is deprecated. Please switch to AddEnterScenarioKey."""
    return AddEnterScenarioKey(builder, EnterScenarioKey)
def AddClearScenarioKey(builder, ClearScenarioKey): builder.PrependUint32Slot(35, ClearScenarioKey, 0)
def EliminateRaidStageExcelAddClearScenarioKey(builder, ClearScenarioKey):
    """This method is deprecated. Please switch to AddClearScenarioKey."""
    return AddClearScenarioKey(builder, ClearScenarioKey)
def AddShowSkillCard(builder, ShowSkillCard): builder.PrependBoolSlot(36, ShowSkillCard, 0)
def EliminateRaidStageExcelAddShowSkillCard(builder, ShowSkillCard):
    """This method is deprecated. Please switch to AddShowSkillCard."""
    return AddShowSkillCard(builder, ShowSkillCard)
def AddBossBGInfoKey(builder, BossBGInfoKey): builder.PrependUint32Slot(37, BossBGInfoKey, 0)
def EliminateRaidStageExcelAddBossBGInfoKey(builder, BossBGInfoKey):
    """This method is deprecated. Please switch to AddBossBGInfoKey."""
    return AddBossBGInfoKey(builder, BossBGInfoKey)
def AddEchelonExtensionType_(builder, EchelonExtensionType_): builder.PrependInt32Slot(38, EchelonExtensionType_, 0)
def EliminateRaidStageExcelAddEchelonExtensionType_(builder, EchelonExtensionType_):
    """This method is deprecated. Please switch to AddEchelonExtensionType_."""
    return AddEchelonExtensionType_(builder, EchelonExtensionType_)
def End(builder): return builder.EndObject()
def EliminateRaidStageExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)