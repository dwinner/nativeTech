cmake_minimum_required(VERSION 3.26)
project(InstallDirectories CXX)

install(DIRECTORY data/ DESTINATION share/calc)

include(GNUInstallDirs)

install(DIRECTORY src/include/calc TYPE INCLUDE
   PATTERN "ignored" EXCLUDE
   PATTERN "extended.h" EXCLUDE
)

install(DIRECTORY etc/ TYPE SYSCONF
   DIRECTORY_PERMISSIONS
      OWNER_READ OWNER_WRITE OWNER_EXECUTE
   PATTERN "nested.conf"
      PERMISSIONS OWNER_READ OWNER_WRITE
)
