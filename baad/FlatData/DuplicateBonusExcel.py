# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class DuplicateBonusExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = DuplicateBonusExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsDuplicateBonusExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # DuplicateBonusExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # DuplicateBonusExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # DuplicateBonusExcel
    def ItemCategory_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # DuplicateBonusExcel
    def ItemId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # DuplicateBonusExcel
    def CharacterId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # DuplicateBonusExcel
    def RewardParcelType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # DuplicateBonusExcel
    def RewardParcelId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # DuplicateBonusExcel
    def RewardParcelAmount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(7)
def DuplicateBonusExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def DuplicateBonusExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddItemCategory_(builder, ItemCategory_): builder.PrependInt32Slot(1, ItemCategory_, 0)
def DuplicateBonusExcelAddItemCategory_(builder, ItemCategory_):
    """This method is deprecated. Please switch to AddItemCategory_."""
    return AddItemCategory_(builder, ItemCategory_)
def AddItemId(builder, ItemId): builder.PrependInt64Slot(2, ItemId, 0)
def DuplicateBonusExcelAddItemId(builder, ItemId):
    """This method is deprecated. Please switch to AddItemId."""
    return AddItemId(builder, ItemId)
def AddCharacterId(builder, CharacterId): builder.PrependInt64Slot(3, CharacterId, 0)
def DuplicateBonusExcelAddCharacterId(builder, CharacterId):
    """This method is deprecated. Please switch to AddCharacterId."""
    return AddCharacterId(builder, CharacterId)
def AddRewardParcelType(builder, RewardParcelType): builder.PrependInt32Slot(4, RewardParcelType, 0)
def DuplicateBonusExcelAddRewardParcelType(builder, RewardParcelType):
    """This method is deprecated. Please switch to AddRewardParcelType."""
    return AddRewardParcelType(builder, RewardParcelType)
def AddRewardParcelId(builder, RewardParcelId): builder.PrependInt64Slot(5, RewardParcelId, 0)
def DuplicateBonusExcelAddRewardParcelId(builder, RewardParcelId):
    """This method is deprecated. Please switch to AddRewardParcelId."""
    return AddRewardParcelId(builder, RewardParcelId)
def AddRewardParcelAmount(builder, RewardParcelAmount): builder.PrependInt64Slot(6, RewardParcelAmount, 0)
def DuplicateBonusExcelAddRewardParcelAmount(builder, RewardParcelAmount):
    """This method is deprecated. Please switch to AddRewardParcelAmount."""
    return AddRewardParcelAmount(builder, RewardParcelAmount)
def End(builder): return builder.EndObject()
def DuplicateBonusExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)