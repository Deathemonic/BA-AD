# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class MinigameTBGSeasonExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = MinigameTBGSeasonExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsMinigameTBGSeasonExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # MinigameTBGSeasonExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # MinigameTBGSeasonExcel
    def EventContentId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def ItemSlot(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def DefaultEchelonHp(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def DefaultItemDiceId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonSlot1CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonSlot2CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonSlot3CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonSlot4CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonSlot1Portrait(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # MinigameTBGSeasonExcel
    def EchelonSlot2Portrait(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # MinigameTBGSeasonExcel
    def EchelonSlot3Portrait(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # MinigameTBGSeasonExcel
    def EchelonSlot4Portrait(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # MinigameTBGSeasonExcel
    def EventUseCostType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(28))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EventUseCostId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(30))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonRevivalCostType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(32))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonRevivalCostId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(34))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EchelonRevivalCostAmount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(36))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EnemyBossHP(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(38))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def EnemyMinionHP(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(40))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def AttackDamage(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(42))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def CriticalAttackDamage(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(44))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def RoundItemSelectLimit(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(46))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def InstantClearRound(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(48))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def MaxHp(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(50))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def MapImagePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(52))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # MinigameTBGSeasonExcel
    def MapNameLocalize(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(54))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # MinigameTBGSeasonExcel
    def StartThemaIndex(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(56))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # MinigameTBGSeasonExcel
    def LoopThemaIndex(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(58))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(28)
def MinigameTBGSeasonExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddEventContentId(builder, EventContentId): builder.PrependInt64Slot(0, EventContentId, 0)
def MinigameTBGSeasonExcelAddEventContentId(builder, EventContentId):
    """This method is deprecated. Please switch to AddEventContentId."""
    return AddEventContentId(builder, EventContentId)
def AddItemSlot(builder, ItemSlot): builder.PrependInt32Slot(1, ItemSlot, 0)
def MinigameTBGSeasonExcelAddItemSlot(builder, ItemSlot):
    """This method is deprecated. Please switch to AddItemSlot."""
    return AddItemSlot(builder, ItemSlot)
def AddDefaultEchelonHp(builder, DefaultEchelonHp): builder.PrependInt32Slot(2, DefaultEchelonHp, 0)
def MinigameTBGSeasonExcelAddDefaultEchelonHp(builder, DefaultEchelonHp):
    """This method is deprecated. Please switch to AddDefaultEchelonHp."""
    return AddDefaultEchelonHp(builder, DefaultEchelonHp)
def AddDefaultItemDiceId(builder, DefaultItemDiceId): builder.PrependInt64Slot(3, DefaultItemDiceId, 0)
def MinigameTBGSeasonExcelAddDefaultItemDiceId(builder, DefaultItemDiceId):
    """This method is deprecated. Please switch to AddDefaultItemDiceId."""
    return AddDefaultItemDiceId(builder, DefaultItemDiceId)
def AddEchelonSlot1CharacterId(builder, EchelonSlot1CharacterId): builder.PrependInt64Slot(4, EchelonSlot1CharacterId, 0)
def MinigameTBGSeasonExcelAddEchelonSlot1CharacterId(builder, EchelonSlot1CharacterId):
    """This method is deprecated. Please switch to AddEchelonSlot1CharacterId."""
    return AddEchelonSlot1CharacterId(builder, EchelonSlot1CharacterId)
def AddEchelonSlot2CharacterId(builder, EchelonSlot2CharacterId): builder.PrependInt64Slot(5, EchelonSlot2CharacterId, 0)
def MinigameTBGSeasonExcelAddEchelonSlot2CharacterId(builder, EchelonSlot2CharacterId):
    """This method is deprecated. Please switch to AddEchelonSlot2CharacterId."""
    return AddEchelonSlot2CharacterId(builder, EchelonSlot2CharacterId)
def AddEchelonSlot3CharacterId(builder, EchelonSlot3CharacterId): builder.PrependInt64Slot(6, EchelonSlot3CharacterId, 0)
def MinigameTBGSeasonExcelAddEchelonSlot3CharacterId(builder, EchelonSlot3CharacterId):
    """This method is deprecated. Please switch to AddEchelonSlot3CharacterId."""
    return AddEchelonSlot3CharacterId(builder, EchelonSlot3CharacterId)
def AddEchelonSlot4CharacterId(builder, EchelonSlot4CharacterId): builder.PrependInt64Slot(7, EchelonSlot4CharacterId, 0)
def MinigameTBGSeasonExcelAddEchelonSlot4CharacterId(builder, EchelonSlot4CharacterId):
    """This method is deprecated. Please switch to AddEchelonSlot4CharacterId."""
    return AddEchelonSlot4CharacterId(builder, EchelonSlot4CharacterId)
def AddEchelonSlot1Portrait(builder, EchelonSlot1Portrait): builder.PrependUOffsetTRelativeSlot(8, flatbuffers.number_types.UOffsetTFlags.py_type(EchelonSlot1Portrait), 0)
def MinigameTBGSeasonExcelAddEchelonSlot1Portrait(builder, EchelonSlot1Portrait):
    """This method is deprecated. Please switch to AddEchelonSlot1Portrait."""
    return AddEchelonSlot1Portrait(builder, EchelonSlot1Portrait)
def AddEchelonSlot2Portrait(builder, EchelonSlot2Portrait): builder.PrependUOffsetTRelativeSlot(9, flatbuffers.number_types.UOffsetTFlags.py_type(EchelonSlot2Portrait), 0)
def MinigameTBGSeasonExcelAddEchelonSlot2Portrait(builder, EchelonSlot2Portrait):
    """This method is deprecated. Please switch to AddEchelonSlot2Portrait."""
    return AddEchelonSlot2Portrait(builder, EchelonSlot2Portrait)
def AddEchelonSlot3Portrait(builder, EchelonSlot3Portrait): builder.PrependUOffsetTRelativeSlot(10, flatbuffers.number_types.UOffsetTFlags.py_type(EchelonSlot3Portrait), 0)
def MinigameTBGSeasonExcelAddEchelonSlot3Portrait(builder, EchelonSlot3Portrait):
    """This method is deprecated. Please switch to AddEchelonSlot3Portrait."""
    return AddEchelonSlot3Portrait(builder, EchelonSlot3Portrait)
def AddEchelonSlot4Portrait(builder, EchelonSlot4Portrait): builder.PrependUOffsetTRelativeSlot(11, flatbuffers.number_types.UOffsetTFlags.py_type(EchelonSlot4Portrait), 0)
def MinigameTBGSeasonExcelAddEchelonSlot4Portrait(builder, EchelonSlot4Portrait):
    """This method is deprecated. Please switch to AddEchelonSlot4Portrait."""
    return AddEchelonSlot4Portrait(builder, EchelonSlot4Portrait)
def AddEventUseCostType(builder, EventUseCostType): builder.PrependInt32Slot(12, EventUseCostType, 0)
def MinigameTBGSeasonExcelAddEventUseCostType(builder, EventUseCostType):
    """This method is deprecated. Please switch to AddEventUseCostType."""
    return AddEventUseCostType(builder, EventUseCostType)
def AddEventUseCostId(builder, EventUseCostId): builder.PrependInt64Slot(13, EventUseCostId, 0)
def MinigameTBGSeasonExcelAddEventUseCostId(builder, EventUseCostId):
    """This method is deprecated. Please switch to AddEventUseCostId."""
    return AddEventUseCostId(builder, EventUseCostId)
def AddEchelonRevivalCostType(builder, EchelonRevivalCostType): builder.PrependInt32Slot(14, EchelonRevivalCostType, 0)
def MinigameTBGSeasonExcelAddEchelonRevivalCostType(builder, EchelonRevivalCostType):
    """This method is deprecated. Please switch to AddEchelonRevivalCostType."""
    return AddEchelonRevivalCostType(builder, EchelonRevivalCostType)
def AddEchelonRevivalCostId(builder, EchelonRevivalCostId): builder.PrependInt64Slot(15, EchelonRevivalCostId, 0)
def MinigameTBGSeasonExcelAddEchelonRevivalCostId(builder, EchelonRevivalCostId):
    """This method is deprecated. Please switch to AddEchelonRevivalCostId."""
    return AddEchelonRevivalCostId(builder, EchelonRevivalCostId)
def AddEchelonRevivalCostAmount(builder, EchelonRevivalCostAmount): builder.PrependInt32Slot(16, EchelonRevivalCostAmount, 0)
def MinigameTBGSeasonExcelAddEchelonRevivalCostAmount(builder, EchelonRevivalCostAmount):
    """This method is deprecated. Please switch to AddEchelonRevivalCostAmount."""
    return AddEchelonRevivalCostAmount(builder, EchelonRevivalCostAmount)
def AddEnemyBossHP(builder, EnemyBossHP): builder.PrependInt32Slot(17, EnemyBossHP, 0)
def MinigameTBGSeasonExcelAddEnemyBossHP(builder, EnemyBossHP):
    """This method is deprecated. Please switch to AddEnemyBossHP."""
    return AddEnemyBossHP(builder, EnemyBossHP)
def AddEnemyMinionHP(builder, EnemyMinionHP): builder.PrependInt32Slot(18, EnemyMinionHP, 0)
def MinigameTBGSeasonExcelAddEnemyMinionHP(builder, EnemyMinionHP):
    """This method is deprecated. Please switch to AddEnemyMinionHP."""
    return AddEnemyMinionHP(builder, EnemyMinionHP)
def AddAttackDamage(builder, AttackDamage): builder.PrependInt32Slot(19, AttackDamage, 0)
def MinigameTBGSeasonExcelAddAttackDamage(builder, AttackDamage):
    """This method is deprecated. Please switch to AddAttackDamage."""
    return AddAttackDamage(builder, AttackDamage)
def AddCriticalAttackDamage(builder, CriticalAttackDamage): builder.PrependInt32Slot(20, CriticalAttackDamage, 0)
def MinigameTBGSeasonExcelAddCriticalAttackDamage(builder, CriticalAttackDamage):
    """This method is deprecated. Please switch to AddCriticalAttackDamage."""
    return AddCriticalAttackDamage(builder, CriticalAttackDamage)
def AddRoundItemSelectLimit(builder, RoundItemSelectLimit): builder.PrependInt32Slot(21, RoundItemSelectLimit, 0)
def MinigameTBGSeasonExcelAddRoundItemSelectLimit(builder, RoundItemSelectLimit):
    """This method is deprecated. Please switch to AddRoundItemSelectLimit."""
    return AddRoundItemSelectLimit(builder, RoundItemSelectLimit)
def AddInstantClearRound(builder, InstantClearRound): builder.PrependInt32Slot(22, InstantClearRound, 0)
def MinigameTBGSeasonExcelAddInstantClearRound(builder, InstantClearRound):
    """This method is deprecated. Please switch to AddInstantClearRound."""
    return AddInstantClearRound(builder, InstantClearRound)
def AddMaxHp(builder, MaxHp): builder.PrependInt32Slot(23, MaxHp, 0)
def MinigameTBGSeasonExcelAddMaxHp(builder, MaxHp):
    """This method is deprecated. Please switch to AddMaxHp."""
    return AddMaxHp(builder, MaxHp)
def AddMapImagePath(builder, MapImagePath): builder.PrependUOffsetTRelativeSlot(24, flatbuffers.number_types.UOffsetTFlags.py_type(MapImagePath), 0)
def MinigameTBGSeasonExcelAddMapImagePath(builder, MapImagePath):
    """This method is deprecated. Please switch to AddMapImagePath."""
    return AddMapImagePath(builder, MapImagePath)
def AddMapNameLocalize(builder, MapNameLocalize): builder.PrependUOffsetTRelativeSlot(25, flatbuffers.number_types.UOffsetTFlags.py_type(MapNameLocalize), 0)
def MinigameTBGSeasonExcelAddMapNameLocalize(builder, MapNameLocalize):
    """This method is deprecated. Please switch to AddMapNameLocalize."""
    return AddMapNameLocalize(builder, MapNameLocalize)
def AddStartThemaIndex(builder, StartThemaIndex): builder.PrependInt32Slot(26, StartThemaIndex, 0)
def MinigameTBGSeasonExcelAddStartThemaIndex(builder, StartThemaIndex):
    """This method is deprecated. Please switch to AddStartThemaIndex."""
    return AddStartThemaIndex(builder, StartThemaIndex)
def AddLoopThemaIndex(builder, LoopThemaIndex): builder.PrependInt32Slot(27, LoopThemaIndex, 0)
def MinigameTBGSeasonExcelAddLoopThemaIndex(builder, LoopThemaIndex):
    """This method is deprecated. Please switch to AddLoopThemaIndex."""
    return AddLoopThemaIndex(builder, LoopThemaIndex)
def End(builder): return builder.EndObject()
def MinigameTBGSeasonExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)