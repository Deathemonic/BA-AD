# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class AniStateData(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = AniStateData()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsAniStateData(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # AniStateData
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # AniStateData
    def StateName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AniStateData
    def StatePrefix(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AniStateData
    def StateNameWithPrefix(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AniStateData
    def Tag(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AniStateData
    def SpeedParameterName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AniStateData
    def SpeedParamter(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # AniStateData
    def StateSpeed(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # AniStateData
    def ClipName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # AniStateData
    def Length(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # AniStateData
    def FrameRate(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # AniStateData
    def IsLooping(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(24))
        if o != 0:
            return bool(self._tab.Get(flatbuffers.number_types.BoolFlags, o + self._tab.Pos))
        return False

    # AniStateData
    def Events(self, j):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            x = self._tab.Vector(o)
            x += flatbuffers.number_types.UOffsetTFlags.py_type(j) * 4
            x = self._tab.Indirect(x)
            from ..FlatData.AniEventData import AniEventData
            obj = AniEventData()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

    # AniStateData
    def EventsLength(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        if o != 0:
            return self._tab.VectorLen(o)
        return 0

    # AniStateData
    def EventsIsNone(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(26))
        return o == 0

def Start(builder): builder.StartObject(12)
def AniStateDataStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddStateName(builder, StateName): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(StateName), 0)
def AniStateDataAddStateName(builder, StateName):
    """This method is deprecated. Please switch to AddStateName."""
    return AddStateName(builder, StateName)
def AddStatePrefix(builder, StatePrefix): builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(StatePrefix), 0)
def AniStateDataAddStatePrefix(builder, StatePrefix):
    """This method is deprecated. Please switch to AddStatePrefix."""
    return AddStatePrefix(builder, StatePrefix)
def AddStateNameWithPrefix(builder, StateNameWithPrefix): builder.PrependUOffsetTRelativeSlot(2, flatbuffers.number_types.UOffsetTFlags.py_type(StateNameWithPrefix), 0)
def AniStateDataAddStateNameWithPrefix(builder, StateNameWithPrefix):
    """This method is deprecated. Please switch to AddStateNameWithPrefix."""
    return AddStateNameWithPrefix(builder, StateNameWithPrefix)
def AddTag(builder, Tag): builder.PrependUOffsetTRelativeSlot(3, flatbuffers.number_types.UOffsetTFlags.py_type(Tag), 0)
def AniStateDataAddTag(builder, Tag):
    """This method is deprecated. Please switch to AddTag."""
    return AddTag(builder, Tag)
def AddSpeedParameterName(builder, SpeedParameterName): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(SpeedParameterName), 0)
def AniStateDataAddSpeedParameterName(builder, SpeedParameterName):
    """This method is deprecated. Please switch to AddSpeedParameterName."""
    return AddSpeedParameterName(builder, SpeedParameterName)
def AddSpeedParamter(builder, SpeedParamter): builder.PrependFloat32Slot(5, SpeedParamter, 0.0)
def AniStateDataAddSpeedParamter(builder, SpeedParamter):
    """This method is deprecated. Please switch to AddSpeedParamter."""
    return AddSpeedParamter(builder, SpeedParamter)
def AddStateSpeed(builder, StateSpeed): builder.PrependFloat32Slot(6, StateSpeed, 0.0)
def AniStateDataAddStateSpeed(builder, StateSpeed):
    """This method is deprecated. Please switch to AddStateSpeed."""
    return AddStateSpeed(builder, StateSpeed)
def AddClipName(builder, ClipName): builder.PrependUOffsetTRelativeSlot(7, flatbuffers.number_types.UOffsetTFlags.py_type(ClipName), 0)
def AniStateDataAddClipName(builder, ClipName):
    """This method is deprecated. Please switch to AddClipName."""
    return AddClipName(builder, ClipName)
def AddLength(builder, Length): builder.PrependFloat32Slot(8, Length, 0.0)
def AniStateDataAddLength(builder, Length):
    """This method is deprecated. Please switch to AddLength."""
    return AddLength(builder, Length)
def AddFrameRate(builder, FrameRate): builder.PrependFloat32Slot(9, FrameRate, 0.0)
def AniStateDataAddFrameRate(builder, FrameRate):
    """This method is deprecated. Please switch to AddFrameRate."""
    return AddFrameRate(builder, FrameRate)
def AddIsLooping(builder, IsLooping): builder.PrependBoolSlot(10, IsLooping, 0)
def AniStateDataAddIsLooping(builder, IsLooping):
    """This method is deprecated. Please switch to AddIsLooping."""
    return AddIsLooping(builder, IsLooping)
def AddEvents(builder, Events): builder.PrependUOffsetTRelativeSlot(11, flatbuffers.number_types.UOffsetTFlags.py_type(Events), 0)
def AniStateDataAddEvents(builder, Events):
    """This method is deprecated. Please switch to AddEvents."""
    return AddEvents(builder, Events)
def StartEventsVector(builder, numElems): return builder.StartVector(4, numElems, 4)
def AniStateDataStartEventsVector(builder, numElems):
    """This method is deprecated. Please switch to Start."""
    return StartEventsVector(builder, numElems)
def End(builder): return builder.EndObject()
def AniStateDataEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)