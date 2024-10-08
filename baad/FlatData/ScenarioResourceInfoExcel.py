# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ScenarioResourceInfoExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ScenarioResourceInfoExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsScenarioResourceInfoExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ScenarioResourceInfoExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ScenarioResourceInfoExcel
    def Id(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioResourceInfoExcel
    def ScenarioModeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioResourceInfoExcel
    def VideoId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioResourceInfoExcel
    def BgmId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # ScenarioResourceInfoExcel
    def AudioName(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioResourceInfoExcel
    def SpinePath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioResourceInfoExcel
    def Ratio(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # ScenarioResourceInfoExcel
    def LobbyAniPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(18))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioResourceInfoExcel
    def MovieCGPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(20))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ScenarioResourceInfoExcel
    def LocalizeId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(22))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Uint32Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(10)
def ScenarioResourceInfoExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddId(builder, Id): builder.PrependInt64Slot(0, Id, 0)
def ScenarioResourceInfoExcelAddId(builder, Id):
    """This method is deprecated. Please switch to AddId."""
    return AddId(builder, Id)
def AddScenarioModeId(builder, ScenarioModeId): builder.PrependInt64Slot(1, ScenarioModeId, 0)
def ScenarioResourceInfoExcelAddScenarioModeId(builder, ScenarioModeId):
    """This method is deprecated. Please switch to AddScenarioModeId."""
    return AddScenarioModeId(builder, ScenarioModeId)
def AddVideoId(builder, VideoId): builder.PrependInt64Slot(2, VideoId, 0)
def ScenarioResourceInfoExcelAddVideoId(builder, VideoId):
    """This method is deprecated. Please switch to AddVideoId."""
    return AddVideoId(builder, VideoId)
def AddBgmId(builder, BgmId): builder.PrependInt64Slot(3, BgmId, 0)
def ScenarioResourceInfoExcelAddBgmId(builder, BgmId):
    """This method is deprecated. Please switch to AddBgmId."""
    return AddBgmId(builder, BgmId)
def AddAudioName(builder, AudioName): builder.PrependUOffsetTRelativeSlot(4, flatbuffers.number_types.UOffsetTFlags.py_type(AudioName), 0)
def ScenarioResourceInfoExcelAddAudioName(builder, AudioName):
    """This method is deprecated. Please switch to AddAudioName."""
    return AddAudioName(builder, AudioName)
def AddSpinePath(builder, SpinePath): builder.PrependUOffsetTRelativeSlot(5, flatbuffers.number_types.UOffsetTFlags.py_type(SpinePath), 0)
def ScenarioResourceInfoExcelAddSpinePath(builder, SpinePath):
    """This method is deprecated. Please switch to AddSpinePath."""
    return AddSpinePath(builder, SpinePath)
def AddRatio(builder, Ratio): builder.PrependInt32Slot(6, Ratio, 0)
def ScenarioResourceInfoExcelAddRatio(builder, Ratio):
    """This method is deprecated. Please switch to AddRatio."""
    return AddRatio(builder, Ratio)
def AddLobbyAniPath(builder, LobbyAniPath): builder.PrependUOffsetTRelativeSlot(7, flatbuffers.number_types.UOffsetTFlags.py_type(LobbyAniPath), 0)
def ScenarioResourceInfoExcelAddLobbyAniPath(builder, LobbyAniPath):
    """This method is deprecated. Please switch to AddLobbyAniPath."""
    return AddLobbyAniPath(builder, LobbyAniPath)
def AddMovieCGPath(builder, MovieCGPath): builder.PrependUOffsetTRelativeSlot(8, flatbuffers.number_types.UOffsetTFlags.py_type(MovieCGPath), 0)
def ScenarioResourceInfoExcelAddMovieCGPath(builder, MovieCGPath):
    """This method is deprecated. Please switch to AddMovieCGPath."""
    return AddMovieCGPath(builder, MovieCGPath)
def AddLocalizeId(builder, LocalizeId): builder.PrependUint32Slot(9, LocalizeId, 0)
def ScenarioResourceInfoExcelAddLocalizeId(builder, LocalizeId):
    """This method is deprecated. Please switch to AddLocalizeId."""
    return AddLocalizeId(builder, LocalizeId)
def End(builder): return builder.EndObject()
def ScenarioResourceInfoExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)