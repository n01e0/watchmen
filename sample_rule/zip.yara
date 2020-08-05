rule IsZIP {
  strings:
     $EOCD_magic = { 50 4B 05 06 }
  condition:
     $EOCD_magic in (0..filesize - 22)
}
