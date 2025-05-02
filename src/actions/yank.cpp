#include <fstream>
#include <stdlib.h>
#include <string>
#include <vector>

#include "../logging/logging.hpp"
#include "../notes/notes.hpp"
#include "actions.hpp"
#include "ncurses.h"

namespace tn
{

static void _get_lines(const std::string &filepath, std::vector<std::string> &lines)
{
	std::string line;
	std::ifstream f;

	f.open(filepath);

	lines.clear();
	while (std::getline(f, line))
		lines.push_back(line);

	f.close();
}

static void _display(int pos, const std::vector<std::string> &lines)
{
	int i;

	for (i = 0; i < lines.size(); ++i) {
		move(i, 0);
		printw("   ");
		move(i, 0);
		if (pos == i)
			printw("->");
		else
			printw(std::to_string(i).c_str());

		move(i, 3);
		printw(lines[i].c_str());

		if (pos == i)
			printw("<-");
		else
			printw("  ");
	}

	refresh();

	return;
}

static int _move_down(int pos, const std::vector<std::string> &lines)
{
	while (pos != lines.size() - 1) {
		++pos;

		if (lines[pos].size() < 2)
			continue;

		return pos;
	}

	return pos;
}

static int _move_up(int pos, const std::vector<std::string> &lines)
{
	while (pos != 0) {
		--pos;

		if (lines[pos].size() < 2)
			continue;

		return pos;
	}

	return pos;
}

static std::string _line_selection(const std::string &filepath)
{
	std::vector<std::string> lines;
	int selected = -1;
	char ch = 0;
	int pos = 0;

	_get_lines(filepath, lines);
	_display(pos, lines);

	while (1) {
		ch = getch();

		switch (ch) {
		case 'y':
			selected = pos;
			break;
		case 'j':
			pos = _move_down(pos, lines);
			break;
		case 'k':
			pos = _move_up(pos, lines);
			break;
		case 'q':
			return "";
		default:
			continue;
		}

		if (selected != -1)
			break;

		_display(pos, lines);
	}

	if (selected == -1)
		return "";

	return lines[selected];
}

static void _save_to_clipboard(const std::string &line)
{
	std::string filt_line;
	std::string command;

	if (line.size() < 2)
		return;

	filt_line = line.substr(0, line.size());

	command = "echo -n '" + filt_line + "' | xclip -selection clipboard";

	// Execute the command
	system(command.c_str());
}

int actions_yank(const params &params, const conf_file &conf)
{
	std::vector<note> notes;
	std::string filepath;
	std::string line;
	int snote;

	initscr();
	curs_set(0);
	noecho();

	notes_get_all(conf.notes_dir, &notes);

	snote = params.note;
	if ((unsigned int)snote >= notes.size()) {
		LOGE("Note index does not exist!");
		return -1;
	}

	filepath = conf.notes_dir + "/" + notes[snote].name + ".txt";

	line = _line_selection(filepath);

	if (!line.empty())
		_save_to_clipboard(line);

	echo();
	curs_set(1);
	endwin();

	return 0;
}

} // namespace tn
