#include <iostream>
#include <sys/prctl.h>
#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

using namespace std;

int main() {
	prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);

	FILE* process;
	process = popen("ruby layer2.rb", "r");

	int grandchild_pid;
	fscanf(process, "%d", &grandchild_pid);

	int child_status = -1;
	cout << "waiting for child..." << endl;
	wait(&child_status);
	cout << "Child exited with " << child_status << endl;

	cout << "waiting for grandchild pid to exit: " << grandchild_pid << endl;

	int status;
	waitpid(grandchild_pid, &status, 0);

	cout << grandchild_pid << " exited with " << WEXITSTATUS(status) << endl;

	return 0;
}
