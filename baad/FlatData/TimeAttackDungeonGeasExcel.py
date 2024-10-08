# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class TimeAttackDungeonGeasExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = TimeAttackDungeonGeasExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsTimeAttackDungeonGeasExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # TimeAttackDungeonGeasExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # TimeAttackDungeonGeasExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def TimeAttackDungeonType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def LocalizeEtcKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def BattleDuration(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def ClearDefaultPoint(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def ClearTimeWeightPoint(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def TimeWeightConst(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def Difficulty(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def RecommandLevel(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def GroundId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        return o == 0

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillLevel(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillLevelAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillLevelLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # TimeAttackDungeonGeasExcel
    def AllyPassiveSkillLevelIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        return o == 0

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillId(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillIdLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillIdIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        return o == 0

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillLevel(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillLevelAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillLevelLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # TimeAttackDungeonGeasExcel
    def EnemyPassiveSkillLevelIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        return o == 0

    # TimeAttackDungeonGeasExcel
    def GeasIconPath(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.String(a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return ""

    # TimeAttackDungeonGeasExcel
    def GeasIconPathLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # TimeAttackDungeonGeasExcel
    def GeasIconPathIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        return o == 0

    # TimeAttackDungeonGeasExcel
    def GeasLocalizeEtcKey(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # TimeAttackDungeonGeasExcel
    def GeasLocalizeEtcKeyAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Uint32Flags, o)
        return 0

    # TimeAttackDungeonGeasExcel
    def GeasLocalizeEtcKeyLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # TimeAttackDungeonGeasExcel
    def GeasLocalizeEtcKeyIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        return o == 0

def Start(builder): builder.StartObject(16)
def TimeAttackDungeonGeasExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def TimeAttackDungeonGeasExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddTimeAttackDungeonType_(builder, TimeAttackDungeonType_): builder.PrependInt32Slot(1, TimeAttackDungeonType_, 0)
def TimeAttackDungeonGeasExcelAddTimeAttackDungeonType_(builder, TimeAttackDungeonType_):
    """This method is deprecated. Please switch to AddTimeAttackDungeonType_."""
    return AddTimeAttackDungeonType_(builder, TimeAttackDungeonType_)
def AddLocalizeEtcKey(builder, LocalizeEtcKey): builder.PrependUint32Slot(2, LocalizeEtcKey, 0)
def TimeAttackDungeonGeasExcelAddLocalizeEtcKey(builder, LocalizeEtcKey):
    """This method is deprecated. Please switch to AddLocalizeEtcKey."""
    return AddLocalizeEtcKey(builder, LocalizeEtcKey)
def AddBattleDuration(builder, BattleDuration): builder.PrependInt64Slot(3, BattleDuration, 0)
def TimeAttackDungeonGeasExcelAddBattleDuration(builder, BattleDuration):
    """This method is deprecated. Please switch to AddBattleDuration."""
    return AddBattleDuration(builder, BattleDuration)
def AddClearDefaultPoint(builder, ClearDefaultPoint): builder.PrependInt64Slot(4, ClearDefaultPoint, 0)
def TimeAttackDungeonGeasExcelAddClearDefaultPoint(builder, ClearDefaultPoint):
    """This method is deprecated. Please switch to AddClearDefaultPoint."""
    return AddClearDefaultPoint(builder, ClearDefaultPoint)
def AddClearTimeWeightPoint(builder, ClearTimeWeightPoint): builder.PrependInt64Slot(5, ClearTimeWeightPoint, 0)
def TimeAttackDungeonGeasExcelAddClearTimeWeightPoint(builder, ClearTimeWeightPoint):
    """This method is deprecated. Please switch to AddClearTimeWeightPoint."""
    return AddClearTimeWeightPoint(builder, ClearTimeWeightPoint)
def AddTimeWeightConst(builder, TimeWeightConst): builder.PrependInt64Slot(6, TimeWeightConst, 0)
def TimeAttackDungeonGeasExcelAddTimeWeightConst(builder, TimeWeightConst):
    """This method is deprecated. Please switch to AddTimeWeightConst."""
    return AddTimeWeightConst(builder, TimeWeightConst)
def AddDifficulty(builder, Difficulty): builder.PrependInt32Slot(7, Difficulty, 0)
def TimeAttackDungeonGeasExcelAddDifficulty(builder, Difficulty):
    """This method is deprecated. Please switch to AddDifficulty."""
    return AddDifficulty(builder, Difficulty)
def AddRecommandLevel(builder, RecommandLevel): builder.PrependInt32Slot(8, RecommandLevel, 0)
def TimeAttackDungeonGeasExcelAddRecommandLevel(builder, RecommandLevel):
    """This method is deprecated. Please switch to AddRecommandLevel."""
    return AddRecommandLevel(builder, RecommandLevel)
def AddGroundId(builder, GroundId): builder.PrependInt64Slot(9, GroundId, 0)
def TimeAttackDungeonGeasExcelAddGroundId(builder, GroundId):
    """This method is deprecated. Please switch to AddGroundId."""
    return AddGroundId(builder, GroundId)
def AddAllyPassiveSkillId(builder, AllyPassiveSkillId): builder.PrependUOffsetTRelativeSlot(10, flatbuffers.number_types.UOffsetTFlags.py_type(AllyPassiveSkillId), 0)
def TimeAttackDungeonGeasExcelAddAllyPassiveSkillId(builder, AllyPassiveSkillId):
    """This method is deprecated. Please switch to AddAllyPassiveSkillId."""
    return AddAllyPassiveSkillId(builder, AllyPassiveSkillId)
def StartAllyPassiveSkillIdVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def TimeAttackDungeonGeasExcelStartAllyPassiveSkillIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartAllyPassiveSkillIdVector(builder, numElems)
def AddAllyPassiveSkillLevel(builder, AllyPassiveSkillLevel): builder.PrependUOffsetTRelativeSlot(11, flatbuffers.number_types.UOffsetTFlags.py_type(AllyPassiveSkillLevel), 0)
def TimeAttackDungeonGeasExcelAddAllyPassiveSkillLevel(builder, AllyPassiveSkillLevel):
    """This method is deprecated. Please switch to AddAllyPassiveSkillLevel."""
    return AddAllyPassiveSkillLevel(builder, AllyPassiveSkillLevel)
def StartAllyPassiveSkillLevelVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def TimeAttackDungeonGeasExcelStartAllyPassiveSkillLevelVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartAllyPassiveSkillLevelVector(builder, numElems)
def AddEnemyPassiveSkillId(builder, EnemyPassiveSkillId): builder.PrependUOffsetTRelativeSlot(12, flatbuffers.number_types.UOffsetTFlags.py_type(EnemyPassiveSkillId), 0)
def TimeAttackDungeonGeasExcelAddEnemyPassiveSkillId(builder, EnemyPassiveSkillId):
    """This method is deprecated. Please switch to AddEnemyPassiveSkillId."""
    return AddEnemyPassiveSkillId(builder, EnemyPassiveSkillId)
def StartEnemyPassiveSkillIdVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def TimeAttackDungeonGeasExcelStartEnemyPassiveSkillIdVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartEnemyPassiveSkillIdVector(builder, numElems)
def AddEnemyPassiveSkillLevel(builder, EnemyPassiveSkillLevel): builder.PrependUOffsetTRelativeSlot(13, flatbuffers.number_types.UOffsetTFlags.py_type(EnemyPassiveSkillLevel), 0)
def TimeAttackDungeonGeasExcelAddEnemyPassiveSkillLevel(builder, EnemyPassiveSkillLevel):
    """This method is deprecated. Please switch to AddEnemyPassiveSkillLevel."""
    return AddEnemyPassiveSkillLevel(builder, EnemyPassiveSkillLevel)
def StartEnemyPassiveSkillLevelVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def TimeAttackDungeonGeasExcelStartEnemyPassiveSkillLevelVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartEnemyPassiveSkillLevelVector(builder, numElems)
def AddGeasIconPath(builder, GeasIconPath): builder.PrependUOffsetTRelativeSlot(14, flatbuffers.number_types.UOffsetTFlags.py_type(GeasIconPath), 0)
def TimeAttackDungeonGeasExcelAddGeasIconPath(builder, GeasIconPath):
    """This method is deprecated. Please switch to AddGeasIconPath."""
    return AddGeasIconPath(builder, GeasIconPath)
def StartGeasIconPathVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def TimeAttackDungeonGeasExcelStartGeasIconPathVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartGeasIconPathVector(builder, numElems)
def AddGeasLocalizeEtcKey(builder, GeasLocalizeEtcKey): builder.PrependUOffsetTRelativeSlot(15, flatbuffers.number_types.UOffsetTFlags.py_type(GeasLocalizeEtcKey), 0)
def TimeAttackDungeonGeasExcelAddGeasLocalizeEtcKey(builder, GeasLocalizeEtcKey):
    """This method is deprecated. Please switch to AddGeasLocalizeEtcKey."""
    return AddGeasLocalizeEtcKey(builder, GeasLocalizeEtcKey)
def StartGeasLocalizeEtcKeyVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def TimeAttackDungeonGeasExcelStartGeasLocalizeEtcKeyVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartGeasLocalizeEtcKeyVector(builder, numElems)
def End(builder): return builder.EndObject()
def TimeAttackDungeonGeasExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)