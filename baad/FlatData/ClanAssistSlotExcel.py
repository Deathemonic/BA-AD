# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ClanAssistSlotExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ClanAssistSlotExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsClanAssistSlotExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ClanAssistSlotExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ClanAssistSlotExcel
    def SlotId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def EchelonType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def SlotNumber(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def AssistTermRewardPeriodFromSec(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def AssistRewardLimit(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def AssistRentRewardDailyMaxCount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def AssistRentalFeeAmount(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ClanAssistSlotExcel
    def AssistRentalFeeAmountStranger(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(8)
def ClanAssistSlotExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddSlotId(builder, SlotId): builder.PrependInt64Slot(0, SlotId, 0)
def ClanAssistSlotExcelAddSlotId(builder, SlotId):
    """This method is deprecated. Please switch to AddSlotId."""
    return AddSlotId(builder, SlotId)
def AddEchelonType_(builder, EchelonType_): builder.PrependInt32Slot(1, EchelonType_, 0)
def ClanAssistSlotExcelAddEchelonType_(builder, EchelonType_):
    """This method is deprecated. Please switch to AddEchelonType_."""
    return AddEchelonType_(builder, EchelonType_)
def AddSlotNumber(builder, SlotNumber): builder.PrependInt64Slot(2, SlotNumber, 0)
def ClanAssistSlotExcelAddSlotNumber(builder, SlotNumber):
    """This method is deprecated. Please switch to AddSlotNumber."""
    return AddSlotNumber(builder, SlotNumber)
def AddAssistTermRewardPeriodFromSec(builder, AssistTermRewardPeriodFromSec): builder.PrependInt64Slot(3, AssistTermRewardPeriodFromSec, 0)
def ClanAssistSlotExcelAddAssistTermRewardPeriodFromSec(builder, AssistTermRewardPeriodFromSec):
    """This method is deprecated. Please switch to AddAssistTermRewardPeriodFromSec."""
    return AddAssistTermRewardPeriodFromSec(builder, AssistTermRewardPeriodFromSec)
def AddAssistRewardLimit(builder, AssistRewardLimit): builder.PrependInt64Slot(4, AssistRewardLimit, 0)
def ClanAssistSlotExcelAddAssistRewardLimit(builder, AssistRewardLimit):
    """This method is deprecated. Please switch to AddAssistRewardLimit."""
    return AddAssistRewardLimit(builder, AssistRewardLimit)
def AddAssistRentRewardDailyMaxCount(builder, AssistRentRewardDailyMaxCount): builder.PrependInt64Slot(5, AssistRentRewardDailyMaxCount, 0)
def ClanAssistSlotExcelAddAssistRentRewardDailyMaxCount(builder, AssistRentRewardDailyMaxCount):
    """This method is deprecated. Please switch to AddAssistRentRewardDailyMaxCount."""
    return AddAssistRentRewardDailyMaxCount(builder, AssistRentRewardDailyMaxCount)
def AddAssistRentalFeeAmount(builder, AssistRentalFeeAmount): builder.PrependInt64Slot(6, AssistRentalFeeAmount, 0)
def ClanAssistSlotExcelAddAssistRentalFeeAmount(builder, AssistRentalFeeAmount):
    """This method is deprecated. Please switch to AddAssistRentalFeeAmount."""
    return AddAssistRentalFeeAmount(builder, AssistRentalFeeAmount)
def AddAssistRentalFeeAmountStranger(builder, AssistRentalFeeAmountStranger): builder.PrependInt64Slot(7, AssistRentalFeeAmountStranger, 0)
def ClanAssistSlotExcelAddAssistRentalFeeAmountStranger(builder, AssistRentalFeeAmountStranger):
    """This method is deprecated. Please switch to AddAssistRentalFeeAmountStranger."""
    return AddAssistRentalFeeAmountStranger(builder, AssistRentalFeeAmountStranger)
def End(builder): return builder.EndObject()
def ClanAssistSlotExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)