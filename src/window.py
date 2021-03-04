# window.py
#
# Copyright 2021 SeaDve
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

from gi.repository import Gtk, Gio, GLib, Handy


@Gtk.Template(resource_path='/io/github/seadve/Kooha/window.ui')
class KoohaWindow(Handy.ApplicationWindow):
    __gtype_name__ = 'KoohaWindow'

    start_record_button = Gtk.Template.Child()  #temp
    stop_record_button = Gtk.Template.Child()
    start_record_button_box = Gtk.Template.Child()
    start_stop_record_button_stack = Gtk.Template.Child()

    fullscreen_mode_toggle = Gtk.Template.Child()
    selection_mode_toggle = Gtk.Template.Child()

    title_stack = Gtk.Template.Child()
    fullscreen_mode_label = Gtk.Template.Child()
    selection_mode_label = Gtk.Template.Child()

    record_audio_toggle = Gtk.Template.Child()
    record_microphone_toggle = Gtk.Template.Child()
    show_pointer_toggle = Gtk.Template.Child()

    menu_button = Gtk.Template.Child()

    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.application = kwargs["application"]

        # setup popover
        builder = Gtk.Builder()
        builder.add_from_resource('/io/github/seadve/Kooha/menu.ui')
        menu_model = builder.get_object('menu')
        popover = Gtk.Popover.new_from_model(self.menu_button, menu_model)
        self.menu_button.set_popover(popover)

        # settings init
        self.record_audio_toggle.set_active(self.application.settings.get_boolean("record-audio"))
        self.record_microphone_toggle.set_active(self.application.settings.get_boolean("record-microphone"))
        self.show_pointer_toggle.set_active(self.application.settings.get_boolean("show-pointer"))

        # dbus init
        self.bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
        self.GNOMEScreencast = Gio.DBusProxy.new_sync(
                    self.bus,
                    Gio.DBusProxyFlags.NONE,
                    None,
                    "org.gnome.Shell.Screencast",
                    "/org/gnome/Shell/Screencast",
                    "org.gnome.Shell.Screencast",
                    None)

        self.GNOMESelectArea = Gio.DBusProxy.new_sync(
                    self.bus,
                    Gio.DBusProxyFlags.NONE,
                    None,
                    "org.gnome.Shell.Screenshot",
                    "/org/gnome/Shell/Screenshot",
                    "org.gnome.Shell.Screenshot",
                    None)




    @Gtk.Template.Callback()
    def on_start_record_button_clicked(self, widget):

        framerate = 30
        pipeline = "queue ! vp8enc min_quantizer=25 max_quantizer=25 cpu-used=3 cq_level=13 deadline=1 threads=3 ! queue ! matroskamux"
        directory = "/home/dave/dave.mkv"

        record_audio = self.application.settings.get_boolean("record-audio")
        record_microphone = self.application.settings.get_boolean("record-microphone")
        show_pointer = self.application.settings.get_boolean("show-pointer")

        delay = int(self.application.settings.get_string("record-delay"))
        video_format = self.application.settings.get_string("video-format")
        saving_location = self.application.settings.get_string("saving-location")

        if self.fullscreen_mode_toggle.get_active():
            self.GNOMEScreencast.call_sync(
                        "Screencast",
                        GLib.Variant.new_tuple(
                            GLib.Variant.new_string(directory),
                            GLib.Variant("a{sv}",
                                {"framerate": GLib.Variant("i", framerate),
                                 "draw-pointer": GLib.Variant("b", show_pointer),
                                 "pipeline": GLib.Variant("s", pipeline)}
                            ),
                        ),
                        Gio.DBusProxyFlags.NONE,
                        -1,
                        None)

        elif self.selection_mode_toggle.get_active():
            coordinates = self.GNOMESelectArea.call_sync("SelectArea", None, Gio.DBusProxyFlags.NONE, -1, None)
            self.GNOMEScreencast.call_sync(
                    "ScreencastArea",
                    GLib.Variant.new_tuple(
                        GLib.Variant("i", coordinates[0]),
                        GLib.Variant("i", coordinates[1]),
                        GLib.Variant("i", coordinates[2]),
                        GLib.Variant("i", coordinates[3]),
                        GLib.Variant.new_string(directory),
                        GLib.Variant("a{sv}",
                            {"framerate": GLib.Variant("i", framerate),
                             "draw-pointer": GLib.Variant("b", show_pointer),
                             "pipeline": GLib.Variant("s", pipeline)}
                        ),
                    ),
                    Gio.DBusProxyFlags.NONE,
                    -1,
                    None)

        self.start_stop_record_button_stack.set_visible_child(self.stop_record_button)


    @Gtk.Template.Callback()
    def on_stop_record_button_clicked(self, widget):

        self.start_stop_record_button_stack.set_visible_child(self.start_record_button_box)

        self.GNOMEScreencast.call_sync(
            "StopScreencast",
            None,
            Gio.DBusCallFlags.NONE,
            -1,
            None)




    @Gtk.Template.Callback()
    def on_fullscreen_mode_clicked(self, widget):
        self.title_stack.set_visible_child(self.fullscreen_mode_label)


    @Gtk.Template.Callback()
    def on_selection_mode_clicked(self, widget):
        self.title_stack.set_visible_child(self.selection_mode_label)





    @Gtk.Template.Callback()
    def on_record_audio_toggled(self, widget):
        if self.record_audio_toggle.get_active():
            self.application.settings.set_boolean("record-audio", True)
        else:
            self.application.settings.set_boolean("record-audio", False)

    @Gtk.Template.Callback()
    def on_record_microphone_toggled(self, widget):
        if self.record_microphone_toggle.get_active():
            self.application.settings.set_boolean("record-microphone", True)
        else:
            self.application.settings.set_boolean("record-microphone", False)

    @Gtk.Template.Callback()
    def on_show_pointer_toggled(self, widget):
        if self.show_pointer_toggle.get_active():
            self.application.settings.set_boolean("show-pointer", True)
        else:
            self.application.settings.set_boolean("show-pointer", False)



