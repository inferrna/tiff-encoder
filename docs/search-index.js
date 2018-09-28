var N = null;var searchIndex = {};
searchIndex["tiff"]={"doc":"A crate for encoding TIFF files. (WIP)","items":[[3,"EndianFile","tiff","A helper structure that provides convenience methods to write to a `fs::File`, being aware of the file's [`Endianness`].",N,N],[3,"Cursor","","Used during the allocation phase of the process of creating a TIFF file.",N,N],[3,"TiffFile","","Representation of a Tagged Image File.",N,N],[3,"IfdChain","","An ordered list of [`Ifd`]s, each pointing to the next one.",N,N],[3,"Ifd","","A structure that holds both an IFD and all the values pointed at by its entries.",N,N],[3,"Offsets","","Represents a list of [`LONG`] values, each pointing to a specific  [`Datablock`].",N,N],[12,"data","","",0,N],[3,"ByteBlock","","[`Datablock`] that consists of a list of bytes.",N,N],[12,"0","","",1,N],[3,"TiffTypeValues","","Represents a list of values of any given [`TiffType`].",N,N],[4,"Endianness","","The byte order used within the TIFF file.",N,N],[13,"II","","Intel byte order, also known as little-endian.",2,N],[13,"MM","","Motorola byte order, also known as big-endian.",2,N],[0,"tiff_type","","Representations of TIFF field data types.",N,N],[3,"BYTE","tiff::tiff_type","8-bit unsigned integer.",N,N],[12,"0","","",3,N],[3,"ASCII","","8-bit byte that contains a 7-bit ASCII code.",N,N],[3,"SHORT","","16-bit (2-byte) unsigned integer.",N,N],[12,"0","","",4,N],[3,"LONG","","32-bit (4-byte) unsigned integer.",N,N],[12,"0","","",5,N],[3,"RATIONAL","","Two LONGs representing, respectively, the numerator and the denominator of a fraction.",N,N],[12,"numerator","","",6,N],[12,"denominator","","",6,N],[3,"SBYTE","","8-bit signed (twos-complement) integer.",N,N],[12,"0","","",7,N],[3,"UNDEFINED","","8-bit byte that may contain anything, depending on the definition of the field.",N,N],[12,"0","","",8,N],[3,"SSHORT","","16-bit (2-byte) signed (twos-complement) integer.",N,N],[12,"0","","",9,N],[3,"SLONG","","32-bit (4-byte) signed (twos-complement) integer.",N,N],[12,"0","","",10,N],[3,"SRATIONAL","","Two SLONGs representing, respectively, the numerator and the denominator of a fraction.",N,N],[12,"numerator","","",11,N],[12,"denominator","","",11,N],[3,"FLOAT","","Single precision (4-byte) IEEE format.",N,N],[12,"0","","",12,N],[3,"DOUBLE","","Double precision (8-byte) IEEE format.",N,N],[12,"0","","",13,N],[8,"TiffType","","A TIFF field type of data.",N,N],[10,"id","","The TIFF 16-bit code that identifies the type.",14,[[],["u16"]]],[10,"size","","The number of bytes occupied by a single value of this type.",14,[[],["u32"]]],[10,"write_to","","The function that writes this type to a given [`EndianFile`].",14,[[["self"],["endianfile"]],["result"]]],[11,"values","","Constructs a [`TiffTypeValues`] of `BYTE`s from a vector of bytes.",3,[[["vec",["u8"]]],["tifftypevalues",["byte"]]]],[11,"single","","Constructs a [`TiffTypeValues`] consisting of a single `BYTE`.",3,[[["u8"]],["tifftypevalues",["byte"]]]],[11,"id","","",3,[[],["u16"]]],[11,"size","","",3,[[],["u32"]]],[11,"write_to","","",3,[[["self"],["endianfile"]],["result"]]],[11,"from_str","","",15,[[["str"]],["tifftypevalues",["ascii"]]]],[11,"values","","",15,[[["vec",["u8"]]],["tifftypevalues",["ascii"]]]],[11,"new","","",15,[[["u8"]],["ascii"]]],[11,"id","","",15,[[],["u16"]]],[11,"size","","",15,[[],["u32"]]],[11,"write_to","","",15,[[["self"],["endianfile"]],["result"]]],[11,"values","","",4,[[["vec",["u16"]]],["tifftypevalues",["short"]]]],[11,"single","","",4,[[["u16"]],["tifftypevalues",["short"]]]],[11,"id","","",4,[[],["u16"]]],[11,"size","","",4,[[],["u32"]]],[11,"write_to","","",4,[[["self"],["endianfile"]],["result"]]],[11,"values","","",5,[[["vec",["u32"]]],["tifftypevalues",["long"]]]],[11,"single","","",5,[[["u32"]],["tifftypevalues",["long"]]]],[11,"id","","",5,[[],["u16"]]],[11,"size","","",5,[[],["u32"]]],[11,"write_to","","",5,[[["self"],["endianfile"]],["result"]]],[11,"values","","",6,[[["vec"]],["tifftypevalues",["rational"]]]],[11,"single","","",6,[[["u32"],["u32"]],["tifftypevalues",["rational"]]]],[11,"id","","",6,[[],["u16"]]],[11,"size","","",6,[[],["u32"]]],[11,"write_to","","",6,[[["self"],["endianfile"]],["result"]]],[11,"values","","",7,[[["vec",["i8"]]],["tifftypevalues",["sbyte"]]]],[11,"single","","",7,[[["i8"]],["tifftypevalues",["sbyte"]]]],[11,"id","","",7,[[],["u16"]]],[11,"size","","",7,[[],["u32"]]],[11,"write_to","","",7,[[["self"],["endianfile"]],["result"]]],[11,"values","","",8,[[["vec",["u8"]]],["tifftypevalues",["undefined"]]]],[11,"single","","",8,[[["u8"]],["tifftypevalues",["undefined"]]]],[11,"id","","",8,[[],["u16"]]],[11,"size","","",8,[[],["u32"]]],[11,"write_to","","",8,[[["self"],["endianfile"]],["result"]]],[11,"values","","",9,[[["vec",["i16"]]],["tifftypevalues",["sshort"]]]],[11,"single","","",9,[[["i16"]],["tifftypevalues",["sshort"]]]],[11,"id","","",9,[[],["u16"]]],[11,"size","","",9,[[],["u32"]]],[11,"write_to","","",9,[[["self"],["endianfile"]],["result"]]],[11,"values","","",10,[[["vec",["i32"]]],["tifftypevalues",["slong"]]]],[11,"single","","",10,[[["i32"]],["tifftypevalues",["slong"]]]],[11,"id","","",10,[[],["u16"]]],[11,"size","","",10,[[],["u32"]]],[11,"write_to","","",10,[[["self"],["endianfile"]],["result"]]],[11,"values","","",11,[[["vec"]],["tifftypevalues",["srational"]]]],[11,"single","","",11,[[["i32"],["i32"]],["tifftypevalues",["srational"]]]],[11,"id","","",11,[[],["u16"]]],[11,"size","","",11,[[],["u32"]]],[11,"write_to","","",11,[[["self"],["endianfile"]],["result"]]],[11,"values","","",12,[[["vec",["f32"]]],["tifftypevalues",["float"]]]],[11,"single","","",12,[[["f32"]],["tifftypevalues",["float"]]]],[11,"id","","",12,[[],["u16"]]],[11,"size","","",12,[[],["u32"]]],[11,"write_to","","",12,[[["self"],["endianfile"]],["result"]]],[11,"values","","",13,[[["vec",["f64"]]],["tifftypevalues",["double"]]]],[11,"single","","",13,[[["f64"]],["tifftypevalues",["double"]]]],[11,"id","","",13,[[],["u16"]]],[11,"size","","",13,[[],["u32"]]],[11,"write_to","","",13,[[["self"],["endianfile"]],["result"]]],[6,"FieldTag","tiff","16-bit identifier of a field entry.",N,N],[8,"FieldValues","","The values contained or pointed at by an IFD Field.",N,N],[8,"Datablock","","Represents a block of data in the file pointed to by a field value, but that isn't part of the field itself.",N,N],[16,"Allocated","","The allocated form of this `Datablock`.",16,N],[10,"size","","The number of bytes occupied by this `Datablock`.",16,[[["self"]],["u32"]]],[10,"allocate","","Allocates this `Datablock`, moving the [`Cursor`] forwards exactly the same number of bytes as returned by [`size(&self)`]. Returns its allocated form, [`Self::Allocated`].",16,N],[8,"AllocatedDatablock","","Represents a [`Datablock`] that already called [`allocate(self, &mut Cursor)`].",N,N],[10,"size","","The number of bytes occupied by this `AllocatedDatablock`.",17,[[["self"]],["u32"]]],[10,"write_to","","Writes this `AllocatedDatablock` to an [`EndianFile`]. The number of bytes written must be exactly same number as returned by [`size(&self)`].",17,[[["self"],["endianfile"]],["result"]]],[8,"SimpleDatablock","","A trait that conveniently implements automatically [`Datablock`] and [`AllocatedDatablock`].",N,N],[10,"size","","The number of bytes occupied by this `Datablock`.",18,[[["self"]],["u32"]]],[10,"write_to","","Writes this `Datablock` to an [`EndianFile`]. The number of bytes  written must be exactly same number as returned by [`size(&self)`].",18,[[["self"],["endianfile"]],["result"]]],[11,"clone","","",2,[[["self"]],["endianness"]]],[11,"allocate","","Allocates a number of bytes to the `Cursor`.",19,[[["self"],["u32"]]]],[11,"allocated_bytes","","Returns the number of already allocated bytes.",19,[[["self"]],["u32"]]],[11,"new","","Creates a new `TiffFile` from an [`IfdChain`].",20,[[["ifdchain"]],["tifffile"]]],[11,"with_endianness","","Returns the same `TiffFile`, but with the specified `Endianness`.",20,[[["self"],["endianness"]],["self"]]],[11,"with_magic_number","","Returns the same `TiffFile`, but with the specified magic number.",20,[[["self"],["u16"]],["self"]]],[11,"write_to","","Writes the `TiffFile`, creating a new file at the given path.",20,[[["self"],["str"]],["result",["file"]]]],[11,"new","","Creates a new `IfdChain` from a vector of [`Ifd`]s.",21,[[["vec",["ifd"]]],["ifdchain"]]],[11,"single","","Creates a new `IfdChain` from a single [`Ifd`].",21,[[["ifd"]],["ifdchain"]]],[11,"new","","Creates a new empty `Ifd`.",22,[[],["ifd"]]],[11,"with_entry","","Returns the same `Ifd`, but adding the given pair of Tag and Values.",22,[[["self"],["fieldtag"],["t"]],["self"]]],[11,"single","","Returns an [`IfdChain`] containing solely this `Ifd`.",22,[[["self"]],["ifdchain"]]],[11,"entry_count","","Returns the number of entries present in this `Ifd`.",22,[[["self"]],["u32"]]],[11,"new","","Creates a new `Offsets` instance from a vector of [`Datablock`]s.",0,[[["vec"]],["self"]]],[11,"count","","",0,[[["self"]],["u32"]]],[11,"size","","",0,[[["self"]],["u32"]]],[11,"allocate","","",0,[[["box"],["cursor"]],["box",["allocatedfieldvalues"]]]],[11,"offsets","","Constructs an [`Offsets`] of `ByteBlock`s from a vector of vectors of bytes.",1,[[["vec",["vec"]]],["offsets",["byteblock"]]]],[11,"size","","",1,[[["self"]],["u32"]]],[11,"write_to","","",1,[[["self"],["endianfile"]],["result"]]],[11,"new","","",23,[[["vec"]],["self"]]],[11,"count","","",23,[[["self"]],["u32"]]],[11,"size","","",23,[[["self"]],["u32"]]],[11,"allocate","","",23,[[["box"],["cursor"]],["box",["allocatedfieldvalues"]]]]],"paths":[[3,"Offsets"],[3,"ByteBlock"],[4,"Endianness"],[3,"BYTE"],[3,"SHORT"],[3,"LONG"],[3,"RATIONAL"],[3,"SBYTE"],[3,"UNDEFINED"],[3,"SSHORT"],[3,"SLONG"],[3,"SRATIONAL"],[3,"FLOAT"],[3,"DOUBLE"],[8,"TiffType"],[3,"ASCII"],[8,"Datablock"],[8,"AllocatedDatablock"],[8,"SimpleDatablock"],[3,"Cursor"],[3,"TiffFile"],[3,"IfdChain"],[3,"Ifd"],[3,"TiffTypeValues"]]};
initSearch(searchIndex);
