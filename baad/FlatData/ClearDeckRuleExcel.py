# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ClearDeckRuleExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ClearDeckRuleExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsClearDeckRuleExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ClearDeckRuleExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ClearDeckRuleExcel
    def ContentType_(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ClearDeckRuleExcel
    def SizeLimit(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(2)
def ClearDeckRuleExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddContentType_(builder, ContentType_): builder.PrependInt32Slot(0, ContentType_, 0)
def ClearDeckRuleExcelAddContentType_(builder, ContentType_):
    """This method is deprecated. Please switch to AddContentType_."""
    return AddContentType_(builder, ContentType_)
def AddSizeLimit(builder, SizeLimit): builder.PrependInt64Slot(1, SizeLimit, 0)
def ClearDeckRuleExcelAddSizeLimit(builder, SizeLimit):
    """This method is deprecated. Please switch to AddSizeLimit."""
    return AddSizeLimit(builder, SizeLimit)
def End(builder): return builder.EndObject()
def ClearDeckRuleExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)