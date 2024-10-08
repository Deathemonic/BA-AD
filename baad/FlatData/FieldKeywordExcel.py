# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FieldKeywordExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FieldKeywordExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFieldKeywordExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FieldKeywordExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FieldKeywordExcel
    def UniqueId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FieldKeywordExcel
    def NameLocalizeKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldKeywordExcel
    def DescriptionLocalizeKey(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # FieldKeywordExcel
    def ImagePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def Start(builder): builder.StartObject(4)
def FieldKeywordExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddUniqueId(builder, UniqueId): builder.PrependInt64Slot(0, UniqueId, 0)
def FieldKeywordExcelAddUniqueId(builder, UniqueId):
    """This method is deprecated. Please switch to AddUniqueId."""
    return AddUniqueId(builder, UniqueId)
def AddNameLocalizeKey(builder, NameLocalizeKey): builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(NameLocalizeKey), 0)
def FieldKeywordExcelAddNameLocalizeKey(builder, NameLocalizeKey):
    """This method is deprecated. Please switch to AddNameLocalizeKey."""
    return AddNameLocalizeKey(builder, NameLocalizeKey)
def AddDescriptionLocalizeKey(builder, DescriptionLocalizeKey): builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(DescriptionLocalizeKey), 0)
def FieldKeywordExcelAddDescriptionLocalizeKey(builder, DescriptionLocalizeKey):
    """This method is deprecated. Please switch to AddDescriptionLocalizeKey."""
    return AddDescriptionLocalizeKey(builder, DescriptionLocalizeKey)
def AddImagePath(builder, ImagePath): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(ImagePath), 0)
def FieldKeywordExcelAddImagePath(builder, ImagePath):
    """This method is deprecated. Please switch to AddImagePath."""
    return AddImagePath(builder, ImagePath)
def End(builder): return builder.EndObject()
def FieldKeywordExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)