cmake_minimum_required(VERSION 3.0.2)
project(rsc_test)

# add_compile_options(-std=c++11)

find_package(catkin REQUIRED)

find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH hello_world1/Cargo.toml)

# find_package(Boost REQUIRED COMPONENTS system)

# catkin_python_setup()

# add_message_files(
#   FILES
#   Message1.msg
#   Message2.msg
# )

# add_service_files(
#   FILES
#   Service1.srv
#   Service2.srv
# )

# add_action_files(
#   FILES
#   Action1.action
#   Action2.action
# )

# generate_messages(
#   DEPENDENCIES
#   std_msgs  # Or other packages containing msgs
# )

# generate_dynamic_reconfigure_options(
#   cfg/DynReconf1.cfg
#   cfg/DynReconf2.cfg
# )

catkin_package(
#  INCLUDE_DIRS include
#  LIBRARIES rsc_test
#  CATKIN_DEPENDS other_catkin_pkg
#  DEPENDS system_lib
)

include_directories(
# include
# ${catkin_INCLUDE_DIRS}
)

# add_library(${PROJECT_NAME}
#   src/${PROJECT_NAME}/test.cpp
# )

# add_dependencies(${PROJECT_NAME} ${${PROJECT_NAME}_EXPORTED_TARGETS} ${catkin_EXPORTED_TARGETS})

add_executable(hello_world2 src/hello_world2.cpp)
# set_target_properties(${PROJECT_NAME}_node PROPERTIES OUTPUT_NAME node PREFIX "")
# add_dependencies(${PROJECT_NAME}_node ${${PROJECT_NAME}_EXPORTED_TARGETS} ${catkin_EXPORTED_TARGETS})
target_link_libraries(hello_world2
  ${catkin_LIBRARIES}
)

# all install targets should use catkin DESTINATION variables
# See http://ros.org/doc/api/catkin/html/adv_user_guide/variables.html

# catkin_install_python(PROGRAMS
#   scripts/my_python_script
#   DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
# )

message(STATUS "##############################")
message(STATUS "${CATKIN_PACKAGE_BIN_DESTINATION}")
message(STATUS ${CATKIN_PACKAGE_BIN_DESTINATION})
message(STATUS "##############################")
corrosion_install(
  TARGETS hello_world1
  RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)

install(TARGETS hello_world2
  RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)

# install(TARGETS ${PROJECT_NAME}
#   ARCHIVE DESTINATION ${CATKIN_PACKAGE_LIB_DESTINATION}
#   LIBRARY DESTINATION ${CATKIN_PACKAGE_LIB_DESTINATION}
#   RUNTIME DESTINATION ${CATKIN_GLOBAL_BIN_DESTINATION}
# )

# install(DIRECTORY include/${PROJECT_NAME}/
#   DESTINATION ${CATKIN_PACKAGE_INCLUDE_DESTINATION}
#   FILES_MATCHING PATTERN "*.h"
#   PATTERN ".svn" EXCLUDE
# )

# install(FILES
#   # myfile1
#   # myfile2
#   DESTINATION ${CATKIN_PACKAGE_SHARE_DESTINATION}
# )

# catkin_add_gtest(${PROJECT_NAME}-test test/test_test.cpp)
# if(TARGET ${PROJECT_NAME}-test)
#   target_link_libraries(${PROJECT_NAME}-test ${PROJECT_NAME})
# endif()

# catkin_add_nosetests(test)
