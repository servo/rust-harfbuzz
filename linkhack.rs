// FIXME: This is a huge hack to find the static version of the library
// instead of the shared. It looks in a very specific place that only has
// relevance to servo.

#[cfg(target_os = "linux")]
#[link_args = "../harfbuzz/src/.libs/libharfbuzz.a -lpango-1.0 -lglib-2.0"]
#[link_args = "-lstdc++"]
#[no_link]
extern mod m { }

#[cfg(target_os = "macos")]
#[link_args = "../harfbuzz/src/.libs/libharfbuzz.a -lstdc++"]
#[no_link]
extern mod m { }
