# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class PropRootMotionFlat(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = PropRootMotionFlat()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsPropRootMotionFlat(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # PropRootMotionFlat
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # PropRootMotionFlat
    def RootMotions(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            x = self._tab.Vector(o)
            x += flatbuffers.number_types.UOffsetTFlags.py_type(j) * 4
            x = self._tab.Indirect(x)
            from ..FlatData.PropMotion import PropMotion
            obj = PropMotion()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

    # PropRootMotionFlat
    def RootMotionsLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # PropRootMotionFlat
    def RootMotionsIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        return o == 0

def Start(builder): builder.StartObject(1)
def PropRootMotionFlatStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddRootMotions(builder, RootMotions): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(RootMotions), 0)
def PropRootMotionFlatAddRootMotions(builder, RootMotions):
    """This method is deprecated. Please switch to AddRootMotions."""
    return AddRootMotions(builder, RootMotions)
def StartRootMotionsVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def PropRootMotionFlatStartRootMotionsVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartRootMotionsVector(builder, numElems)
def End(builder): return builder.EndObject()
def PropRootMotionFlatEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)