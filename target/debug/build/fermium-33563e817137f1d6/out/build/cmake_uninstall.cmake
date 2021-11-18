if (NOT EXISTS "/Users/Adebas/Desktop/RustMath/target/debug/build/fermium-33563e817137f1d6/out/build/install_manifest.txt")
    message(FATAL_ERROR "Cannot find install manifest: \"/Users/Adebas/Desktop/RustMath/target/debug/build/fermium-33563e817137f1d6/out/build/install_manifest.txt\"")
endif(NOT EXISTS "/Users/Adebas/Desktop/RustMath/target/debug/build/fermium-33563e817137f1d6/out/build/install_manifest.txt")

file(READ "/Users/Adebas/Desktop/RustMath/target/debug/build/fermium-33563e817137f1d6/out/build/install_manifest.txt" files)
string(REGEX REPLACE "\n" ";" files "${files}")
foreach (file ${files})
    message(STATUS "Uninstalling \"$ENV{DESTDIR}${file}\"")
    execute_process(
        COMMAND /opt/homebrew/Cellar/cmake/3.21.4/bin/cmake -E remove "$ENV{DESTDIR}${file}"
        OUTPUT_VARIABLE rm_out
        RESULT_VARIABLE rm_retval
    )
    if(NOT ${rm_retval} EQUAL 0)
        message(FATAL_ERROR "Problem when removing \"$ENV{DESTDIR}${file}\"")
    endif (NOT ${rm_retval} EQUAL 0)
endforeach(file)

