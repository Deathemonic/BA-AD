# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class HpBarAbbreviationExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = HpBarAbbreviationExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsHpBarAbbreviationExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # HpBarAbbreviationExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # HpBarAbbreviationExcel
    def MonsterLv(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # HpBarAbbreviationExcel
    def StandardHpBar(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # HpBarAbbreviationExcel
    def RaidBossHpBar(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(3)
def HpBarAbbreviationExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddMonsterLv(builder, MonsterLv): builder.PrependInt32Slot(0, MonsterLv, 0)
def HpBarAbbreviationExcelAddMonsterLv(builder, MonsterLv):
    """This method is deprecated. Please switch to AddMonsterLv."""
    return AddMonsterLv(builder, MonsterLv)
def AddStandardHpBar(builder, StandardHpBar): builder.PrependInt32Slot(1, StandardHpBar, 0)
def HpBarAbbreviationExcelAddStandardHpBar(builder, StandardHpBar):
    """This method is deprecated. Please switch to AddStandardHpBar."""
    return AddStandardHpBar(builder, StandardHpBar)
def AddRaidBossHpBar(builder, RaidBossHpBar): builder.PrependInt32Slot(2, RaidBossHpBar, 0)
def HpBarAbbreviationExcelAddRaidBossHpBar(builder, RaidBossHpBar):
    """This method is deprecated. Please switch to AddRaidBossHpBar."""
    return AddRaidBossHpBar(builder, RaidBossHpBar)
def End(builder): return builder.EndObject()
def HpBarAbbreviationExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)