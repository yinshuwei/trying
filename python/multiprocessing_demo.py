import multiprocessing
import time


def do_task(param):
    ''' 执行单个任务的代码 '''

    print(param)
    time.sleep(1)


def do_tasks(p_count):
    ''' 多进程管理, p_count 进程个数'''

    all_params = [1, 2, 3, 4, 5, 3, 4, 5, 6, 7, 8, 9, 8, 23] #所有任务参数

    p = multiprocessing.Pool(processes=p_count)
    for param in all_params:
        p.apply_async(do_task, args=(param,))
    print('等待所有任务执行完毕...')
    p.close()
    p.join()
    print('所有任务执行完毕!')


if __name__ == '__main__':
    do_tasks(3)
