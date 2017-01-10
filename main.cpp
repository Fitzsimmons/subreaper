#include <iostream>
#include <sys/prctl.h>
#include <stdio.h>
#include <sys/wait.h>
#include <unistd.h>

using namespace std;

int main() {

	int res = prctl(PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
	cout << "res: " << res << endl;

	FILE* process;
	process = popen("ruby layer2.rb", "r");

	int grandchild_pid;
	fscanf(process, "%d", &grandchild_pid);

	int child_status = -1;
	cout << "waiting for child..." << endl;
	wait(&child_status);
	cout << "Child exited with " << child_status << endl;

	cout << "waiting for grandchild pid: " << grandchild_pid << endl;

	siginfo_t info = {};
	waitid(P_PID, grandchild_pid, &info, WEXITED);

	cout << info.si_pid << " exited with " << info.si_status << endl;

	return 0;
}
