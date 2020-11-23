cmake_minimum_required(VERSION 3.0.2)
project(rsc_test)

# add_compile_options(-std=c++11)

find_package(catkin REQUIRED COMPONENTS
  roscpp
  std_msgs
)

find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH rust_src/Cargo.toml)

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
  CATKIN_DEPENDS roscpp rospy std_msgs
#  DEPENDS system_lib
)

include_directories(
# include
  ${catkin_INCLUDE_DIRS}
)

# add_library(${PROJECT_NAME}
#   src/${PROJECT_NAME}/test.cpp
# )

# add_dependencies(${PROJECT_NAME} ${${PROJECT_NAME}_EXPORTED_TARGETS} ${catkin_EXPORTED_TARGETS})

add_executable(hello_world2 src/hello_world2.cpp)
target_link_libraries(hello_world2 ${catkin_LIBRARIES})

add_executable(test_pub_sub src/test_pub_sub.cpp)
target_link_libraries(test_pub_sub ${catkin_LIBRARIES})

# add_executable(image_sub_pub src/image_sub_pub.cpp)
# target_link_libraries(image_sub_pub ${catkin_LIBRARIES})

# all install targets should use catkin DESTINATION variables
# See http://ros.org/doc/api/catkin/html/adv_user_guide/variables.html

# catkin_install_python(PROGRAMS
#   scripts/my_python_script
#   DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
# )

function(rust_target target_name)
  add_custom_target(${target_name}_copy_to_devel ALL  # TARGET hello_world1 POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:${target_name}> ${CATKIN_DEVEL_PREFIX}/lib/${PROJECT_NAME}/${target_name}
  )
  add_dependencies(${target_name}_copy_to_devel ${target_name})
  corrosion_install(
    TARGETS ${target_name}
    RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
  )
endfunction(rust_target)

rust_target(hello_world1)
rust_target(log_test)
rust_target(image_sub_pub)

# target is imported and does not build here
# add_custom_command(TARGET hello_world1 POST_BUILD
#   COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:hello_world1> ${CATKIN_DEVEL_PREFIX}/lib/${PROJECT_NAME}/hello_world1
# )

# This works
# add_custom_target(copy_to_devel ALL  # TARGET hello_world1 POST_BUILD
#   COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:hello_world1> ${CATKIN_DEVEL_PREFIX}/lib/${PROJECT_NAME}/hello_world1
# )
# add_dependencies(copy_to_devel hello_world1)
# corrosion_install(
#   TARGETS hello_world1
#   RUNTIME DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
# )

# Doesn't run after build
# configure_file(${CMAKE_CURRENT_BINARY_DIR}/hello_world1 ${CATKIN_DEVEL_PREFIX}/lib/${PROJECT_NAME}/hello_world1 COPYONLY)


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
