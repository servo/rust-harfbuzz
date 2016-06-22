project(harfbuzz)
cmake_minimum_required(VERSION 2.6)

# Needed to avoid error
set(DUMMY ${CMAKE_BUILD_TYPE})

set(HARFBUZZ_SOURCES
  # HBSOURCES
  src/hb-atomic-private.hh
  src/hb-blob.cc
  src/hb-buffer-deserialize-json.hh
  src/hb-buffer-deserialize-text.hh
  src/hb-buffer-private.hh
  src/hb-buffer-serialize.cc
  src/hb-buffer.cc
  src/hb-cache-private.hh
  src/hb-common.cc
  src/hb-face-private.hh
  src/hb-face.cc
  src/hb-font-private.hh
  src/hb-font.cc
  src/hb-mutex-private.hh
  src/hb-object-private.hh
  src/hb-open-file-private.hh
  src/hb-open-type-private.hh
  src/hb-ot-cmap-table.hh
  src/hb-ot-glyf-table.hh
  src/hb-ot-head-table.hh
  src/hb-ot-hhea-table.hh
  src/hb-ot-hmtx-table.hh
  src/hb-ot-maxp-table.hh
  src/hb-ot-name-table.hh
  src/hb-ot-tag.cc
  src/hb-private.hh
  src/hb-set-private.hh
  src/hb-set.cc
  src/hb-shape.cc
  src/hb-shape-plan-private.hh
  src/hb-shape-plan.cc
  src/hb-shaper-list.hh
  src/hb-shaper-impl-private.hh
  src/hb-shaper-private.hh
  src/hb-shaper.cc
  src/hb-unicode-private.hh
  src/hb-unicode.cc
  src/hb-utf-private.hh
  src/hb-warning.cc
  # HBHEADERS
  src/hb.h
  src/hb-blob.h
  src/hb-buffer.h
  src/hb-common.h
  src/hb-deprecated.h
  src/hb-face.h
  src/hb-font.h
  src/hb-set.h
  src/hb-shape.h
  src/hb-shape-plan.h
  src/hb-unicode.h
  # HBNODISTHEADERS
  src/hb-version.h
  # UCDN HBSOURCES
  src/hb-ucdn.cc
  # UCDN
  src/hb-ucdn/ucdn.h
  src/hb-ucdn/ucdn.c
  src/hb-ucdn/unicodedata_db.h
  # OT HBSOUCES
  src/hb-ot-font.cc
  src/hb-ot-layout.cc
  src/hb-ot-layout-common-private.hh
  src/hb-ot-layout-gdef-table.hh
  src/hb-ot-layout-gpos-table.hh
  src/hb-ot-layout-gsubgpos-private.hh
  src/hb-ot-layout-gsub-table.hh
  src/hb-ot-layout-jstf-table.hh
  src/hb-ot-layout-private.hh
  src/hb-ot-map.cc
  src/hb-ot-map-private.hh
  src/hb-ot-shape.cc
  src/hb-ot-shape-complex-arabic.cc
  src/hb-ot-shape-complex-arabic-fallback.hh
  src/hb-ot-shape-complex-arabic-private.hh
  src/hb-ot-shape-complex-arabic-table.hh
  src/hb-ot-shape-complex-arabic-win1256.hh
  src/hb-ot-shape-complex-default.cc
  src/hb-ot-shape-complex-hangul.cc
  src/hb-ot-shape-complex-hebrew.cc
  src/hb-ot-shape-complex-indic.cc
  src/hb-ot-shape-complex-indic-machine.hh
  src/hb-ot-shape-complex-indic-private.hh
  src/hb-ot-shape-complex-indic-table.cc
  src/hb-ot-shape-complex-myanmar.cc
  src/hb-ot-shape-complex-myanmar-machine.hh
  src/hb-ot-shape-complex-thai.cc
  src/hb-ot-shape-complex-tibetan.cc
  src/hb-ot-shape-complex-use.cc
  src/hb-ot-shape-complex-use-machine.hh
  src/hb-ot-shape-complex-use-private.hh
  src/hb-ot-shape-complex-use-table.cc
  src/hb-ot-shape-complex-private.hh
  src/hb-ot-shape-normalize-private.hh
  src/hb-ot-shape-normalize.cc
  src/hb-ot-shape-fallback-private.hh
  src/hb-ot-shape-fallback.cc
  src/hb-ot-shape-private.hh
  # OT HBHEADERS
  src/hb-ot.h
  src/hb-ot-font.h
  src/hb-ot-layout.h
  src/hb-ot-shape.h
  src/hb-ot-tag.h
  )

include_directories(src/hb-ucdn)
add_definitions(-DHAVE_OT -DHAVE_UCDN)

if(WIN32 AND NOT MINGW)
  add_definitions(-D_CRT_SECURE_NO_WARNINGS)
endif()

add_library(harfbuzz STATIC ${HARFBUZZ_SOURCES})
install(TARGETS harfbuzz ARCHIVE DESTINATION .)
