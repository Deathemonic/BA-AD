# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ShortcutTypeExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ShortcutTypeExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsShortcutTypeExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ShortcutTypeExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ShortcutTypeExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ShortcutTypeExcel
    def IsAscending(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # ShortcutTypeExcel
    def ContentType(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            a = self._tab.Vector(o)
            return self._tab.Get(flatbuffers.number_types.Int32Flags, a + flatbuffers.number_types.UOffsetTFlags.py_type(j * 4))
        return 0

    # ShortcutTypeExcel
    def ContentTypeAsNumpy(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.GetVectorAsNumpy(flatbuffers.number_types.Int32Flags, o)
        return 0

    # ShortcutTypeExcel
    def ContentTypeLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # ShortcutTypeExcel
    def ContentTypeIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        return o == 0

def Start(builder): builder.StartObject(3)
def ShortcutTypeExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def ShortcutTypeExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddIsAscending(builder, IsAscending): builder.PrependBoolSlot(1, IsAscending, 0)
def ShortcutTypeExcelAddIsAscending(builder, IsAscending):
    """This method is deprecated. Please switch to AddIsAscending."""
    return AddIsAscending(builder, IsAscending)
def AddContentType(builder, ContentType): builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(ContentType), 0)
def ShortcutTypeExcelAddContentType(builder, ContentType):
    """This method is deprecated. Please switch to AddContentType."""
    return AddContentType(builder, ContentType)
def StartContentTypeVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def ShortcutTypeExcelStartContentTypeVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartContentTypeVector(builder, numElems)
def End(builder): return builder.EndObject()
def ShortcutTypeExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)