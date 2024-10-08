# automatically generated by the FlatBuffers compiler, do not modify

# namespace: FlatData

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class FurnitureTemplateElementExcel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = FurnitureTemplateElementExcel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsFurnitureTemplateElementExcel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # FurnitureTemplateElementExcel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # FurnitureTemplateElementExcel
    def FurnitureTemplateId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FurnitureTemplateElementExcel
    def FurnitureId(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

    # FurnitureTemplateElementExcel
    def Location(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # FurnitureTemplateElementExcel
    def PositionX(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # FurnitureTemplateElementExcel
    def PositionY(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # FurnitureTemplateElementExcel
    def Rotation(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

    # FurnitureTemplateElementExcel
    def Order(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(16))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int64Flags, o + self._tab.Pos)
        return 0

def Start(builder): builder.StartObject(7)
def FurnitureTemplateElementExcelStart(builder):
    """This method is deprecated. Please switch to Start."""
    return Start(builder)
def AddFurnitureTemplateId(builder, FurnitureTemplateId): builder.PrependInt64Slot(0, FurnitureTemplateId, 0)
def FurnitureTemplateElementExcelAddFurnitureTemplateId(builder, FurnitureTemplateId):
    """This method is deprecated. Please switch to AddFurnitureTemplateId."""
    return AddFurnitureTemplateId(builder, FurnitureTemplateId)
def AddFurnitureId(builder, FurnitureId): builder.PrependInt64Slot(1, FurnitureId, 0)
def FurnitureTemplateElementExcelAddFurnitureId(builder, FurnitureId):
    """This method is deprecated. Please switch to AddFurnitureId."""
    return AddFurnitureId(builder, FurnitureId)
def AddLocation(builder, Location): builder.PrependInt32Slot(2, Location, 0)
def FurnitureTemplateElementExcelAddLocation(builder, Location):
    """This method is deprecated. Please switch to AddLocation."""
    return AddLocation(builder, Location)
def AddPositionX(builder, PositionX): builder.PrependFloat32Slot(3, PositionX, 0.0)
def FurnitureTemplateElementExcelAddPositionX(builder, PositionX):
    """This method is deprecated. Please switch to AddPositionX."""
    return AddPositionX(builder, PositionX)
def AddPositionY(builder, PositionY): builder.PrependFloat32Slot(4, PositionY, 0.0)
def FurnitureTemplateElementExcelAddPositionY(builder, PositionY):
    """This method is deprecated. Please switch to AddPositionY."""
    return AddPositionY(builder, PositionY)
def AddRotation(builder, Rotation): builder.PrependFloat32Slot(5, Rotation, 0.0)
def FurnitureTemplateElementExcelAddRotation(builder, Rotation):
    """This method is deprecated. Please switch to AddRotation."""
    return AddRotation(builder, Rotation)
def AddOrder(builder, Order): builder.PrependInt64Slot(6, Order, 0)
def FurnitureTemplateElementExcelAddOrder(builder, Order):
    """This method is deprecated. Please switch to AddOrder."""
    return AddOrder(builder, Order)
def End(builder): return builder.EndObject()
def FurnitureTemplateElementExcelEnd(builder):
    """This method is deprecated. Please switch to End."""
    return End(builder)