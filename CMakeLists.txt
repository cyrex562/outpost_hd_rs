cmake_minimum_required(VERSION 3.13)
project(outpost_hd)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 17)


set(SOURCE_FILES src/main.cpp src/animation_set.cpp src/button.cpp src/cheatmenu.cpp src/checkbox.cpp src/color.cpp src/combobox.cpp src/common.cpp src/configuration.cpp src/control.cpp src/crimeexecution.cpp src/crimerateupdate.cpp src/detailmap.cpp src/dictionary.cpp src/diggerdirection.cpp src/directionoffset.cpp src/display_desc.cpp src/eventhandler.cpp src/factory.cpp src/factorylistbox.cpp src/factoryproduction.cpp src/factoryreport.cpp src/fade.cpp src/fileio.cpp src/filesystem.cpp src/font.cpp src/fpscounter.cpp src/mapviewstate.cpp src/image.cpp src/researchtracker.cpp src/mapviewstateio.cpp src/productpool.cpp src/mineoperationswindow.cpp src/renderer.cpp src/productcatalogue.cpp src/listbox.cpp src/mapviewstateevent.cpp src/populationtable.cpp src/mapviewstatehelper.cpp src/mixer.cpp src/parserhelper.cpp src/mapviewstateturn.cpp src/minefacility.cpp src/populationpanel.cpp src/radiobutton.cpp src/game.cpp src/mine.cpp src/mainreportsuistate.cpp src/majoreventannouncement.cpp src/icongrid.cpp src/rectangle_skin.cpp src/renderer_window.cpp src/point.cpp src/mapcoordinate.cpp src/minereport.cpp src/messagebox.cpp src/planet.cpp src/music.cpp src/math_utils.cpp src/rectangle.cpp src/populationpool.cpp src/notificationwindow.cpp src/mapobject.cpp src/researchreport.cpp src/resourceinfobar.cpp src/micropather.cpp src/resourcebreakdownpanel.cpp src/mixer_sdl.cpp src/mapviewstateui.cpp src/mapview.cpp src/radiobuttongroup.cpp src/robodigger.cpp src/navcontrol.cpp src/mapviewstatedraw.cpp src/notificationarea.cpp src/robodozer.cpp src/planetselectstate.cpp src/productlistbox.cpp src/graphwalker.cpp src/mixer_null.cpp src/gameoverdialog.cpp src/iohelper.cpp src/minimap.cpp src/renderer_opengl.cpp src/mainmenustate.cpp src/listboxbase.cpp src/gamestate.cpp src/gameoptionsdialog.cpp src/population.cpp src/label.cpp src/robominer.cpp src/robot.cpp src/robotdeploymentsummary.cpp src/robotinspector.cpp src/robotpool.cpp src/satellitesreport.cpp src/scrollbar.cpp src/shellopenpath.cpp src/sound.cpp src/spaceportsreport.cpp src/splashstate.cpp src/sprite.cpp src/statemanager.cpp src/stringtable.cpp src/stringutils.cpp src/structure.cpp src/structurecatalogue.cpp src/structureinspector.cpp src/structurelistbox.cpp src/structuremanager.cpp src/structuretracker.cpp src/technologycatalog.cpp src/textarea.cpp src/textcontrol.cpp src/textfield.cpp src/textrender.cpp src/tile.cpp src/tileinspector.cpp src/tilemap.cpp src/timer.cpp src/tooltip.cpp src/trig.cpp src/uicontainer.cpp src/version.cpp src/warehouseinspector.cpp src/warehousereport.cpp src/window.cpp src/windowstack.cpp src/x.cpp src/xmlattribute.cpp src/xmlattributeset.cpp src/xmlbase.cpp src/xmlcomment.cpp src/xmldocument.cpp src/xmlelement.cpp src/xmlhandle.cpp src/xmlmemorybuffer.cpp src/xmlnode.cpp src/xmlparser.cpp src/xmlserializer.cpp src/xmltext.cpp src/xmlunknown.cpp)

link_directories(vcpkg_installed/x64-windows-static/lib)

add_executable(outpost ${SOURCE_FILES})

include_directories(src vcpkg_installed/x64-windows-static/include vcpkg_installed/x64-windows-static/include/brotli vcpkg_installed/x64-windows-static/include/EGL vcpkg_installed/x64-windows-static/include/freetype
        vcpkg_installed/x64-windows-static/include/GL
        vcpkg_installed/x64-windows-static/include/GLES
        vcpkg_installed/x64-windows-static/include/GLES2
        vcpkg_installed/x64-windows-static/include/GLES3
        vcpkg_installed/x64-windows-static/include/GLSC
        vcpkg_installed/x64-windows-static/include/GLSC2
        vcpkg_installed/x64-windows-static/include/gmock
        vcpkg_installed/x64-windows-static/include/gtest
        vcpkg_installed/x64-windows-static/include/KHR
        vcpkg_installed/x64-windows-static/include/libpng16
        vcpkg_installed/x64-windows-static/include/ogg
        vcpkg_installed/x64-windows-static/include/SDL2
        vcpkg_installed/x64-windows-static/include/vorbis)

set(ZLIB_LIBRARY vcpkg_installed/x64-windows-static/lib)
set(ZLIB_INCLUDE_DIR vcpkg_installed/x64-windows-static/include)
set(ZLIB_DIR vcpkg_installed/x64-windows-static/share/zlib)
#find_package(ZLIB)

set(PNG_PNG_INCLUDE_DIR vcpkg_installed/x64-windows-static/include/libpng16)
set(PNG_LIBRARY vcpkg_installed/x64-windows-static/lib)
set(PNG_DIR vcpkg_installed/x64-windows-static/share/libpng)
##find_package(PNG)
#find_library()
#target_link_libraries(outpost PRIVATE PNG)

#set(GLEW_INCLUDE_DIRS vcpkg_installed/x64-windows-static/include)
#set(GLEW_LIBRARIES vcpkg_installed/x64-windows-static/lib)
set(GLEW_DIR vcpkg_installed/x64-windows-static/share/glew)
find_package(GLEW REQUIRED)
target_link_libraries(outpost PRIVATE GLEW::GLEW)

set(SDL2_DIR vcpkg_installed/x64-windows-static/share/sdl2)
find_package(SDL2 CONFIG REQUIRED)
target_link_libraries(outpost
        PRIVATE
        $<TARGET_NAME_IF_EXISTS:SDL2::SDL2main>
        $<IF:$<TARGET_EXISTS:SDL2::SDL2>,SDL2::SDL2,SDL2::SDL2-static>
        )




set(SDL2_image_DIR vcpkg_installed/x64-windows-static/share/SDL2_image)
find_package(SDL2_image CONFIG REQUIRED)
target_link_libraries(outpost PRIVATE $<IF:$<TARGET_EXISTS:SDL2_image::SDL2_image>,SDL2_image::SDL2_image,SDL2_image::SDL2_image-static> )

set(SDL2_mixer_DIR vcpkg_installed/x64-windows-static/share/SDL2_mixer)
find_package(SDL2_mixer CONFIG REQUIRED)
target_link_libraries(outpost PRIVATE $<IF:$<TARGET_EXISTS:SDL2_mixer::SDL2_mixer>,SDL2_mixer::SDL2_mixer,SDL2_mixer::SDL2_mixer-static> )

set(SDL2_ttf_DIR vcpkg_installed/x64-windows-static/share/SDL2_ttf)
find_package(SDL2_ttf CONFIG REQUIRED)
target_link_libraries(outpost PRIVATE $<IF:$<TARGET_EXISTS:SDL2_ttf::SDL2_ttf>,SDL2_ttf::SDL2_ttf,SDL2_ttf::SDL2_ttf-static> )


