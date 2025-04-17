#include <gtk/gtk.h>

static void on_open_file(GtkButton *button, gpointer user_data) {
    GtkWindow *parent_window = GTK_WINDOW(user_data);

    GtkFileDialog *dialog = gtk_file_dialog_new();
    gtk_file_dialog_set_title(dialog, "Select a media file");

    gtk_file_dialog_open(dialog, parent_window, NULL, NULL, NULL);
}

static void on_convert_clicked(GtkButton *button, gpointer user_data) {
    g_print("Conversion started... (not implemented yet)\n");
}

static void activate(GtkApplication *app, gpointer user_data) {
    GtkWidget *window;
    GtkWidget *vbox;
    GtkWidget *open_button;
    GtkWidget *convert_button;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Offline Media Converter");
    gtk_window_set_default_size(GTK_WINDOW(window), 500, 300);
    gtk_window_set_icon_name(GTK_WINDOW(window), "media-converter");

    vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
    gtk_widget_set_margin_top(vbox, 20);
    gtk_widget_set_margin_bottom(vbox, 20);
    gtk_widget_set_margin_start(vbox, 20);
    gtk_widget_set_margin_end(vbox, 20);
    gtk_window_set_child(GTK_WINDOW(window), vbox);

    // Open file button
    open_button = gtk_button_new_with_label("Choose Media File");
    g_signal_connect(open_button, "clicked", G_CALLBACK(on_open_file), window);
    gtk_box_append(GTK_BOX(vbox), open_button);

    // Convert button
    convert_button = gtk_button_new_with_label("Convert");
    g_signal_connect(convert_button, "clicked", G_CALLBACK(on_convert_clicked), NULL);
    gtk_box_append(GTK_BOX(vbox), convert_button);

    gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char **argv) {
    GtkApplication *app;
    int status;

    app = gtk_application_new("com.offline.converter", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}