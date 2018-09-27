initSidebarItems({"enum":[["Endianness","The byte order used within the TIFF file."]],"mod":[["tiff_type","Representations of TIFF field data types."]],"struct":[["ByteBlock","[`Datablock`] that consists of a list of bytes."],["Cursor","Used during the Allocation Phase of the `TiffFile` writting."],["EndianFile","A helper structure that provides convenience methods to write to a `fs::File`, being aware of the file's [`Endianness`]."],["Ifd","A structure that holds both an IFD and all the values pointed at by its entries."],["IfdChain","An ordered list of [`Ifd`]s, each pointing to the next one."],["Offsets","Represents a list of [`LONG`] values, each pointing to a specific  [`Datablock`]."],["TiffFile","Representation of a Tagged Image File."],["TiffTypeValues","Represents a list of values of any given [`TiffType`]."]],"trait":[["AllocatedDatablock","Represents a [`Datablock`] that already called [`allocate(self, &mut Cursor)`]."],["Datablock","Represents a block of data in the file pointed to by a field value, but that isn't part of the field itself."],["FieldValues","The values contained or pointed at by an IFD Field."],["SimpleDatablock","A trait that conveniently implements automatically [`Datablock`] and [`AllocatedDatablock`]."]],"type":[["FieldTag","16-bit identifier of a field entry."]]});