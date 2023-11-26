import serial


class PortReader:
    portx = "COM5"
    bps = 9600
    timex = 5

    def __init__(self):
        self.ser = serial.Serial(self.portx, self.bps, timeout=self.timex)

    def __iter__(self):
        return self

    def __next__(self):
        while a := self.ser.read(4):
            if a == b"\xaa\xaa\x20\x02":
                break
        read = self.ser.read(32)
        attention = read[-4]
        meditation = read[-2]
        delta = (read[3] << 16) | (read[4] << 8) | read[5]
        theta = (read[6] << 16) | (read[7] << 8) | read[8]
        low_alpha = (read[9] << 16) | (read[10] << 8) | read[11]
        high_alpha = (read[12] << 16) | (read[13] << 8) | read[14]
        low_beta = (read[15] << 16) | (read[16] << 8) | read[17]
        high_beta = (read[18] << 16) | (read[19] << 8) | read[20]
        low_gamma = (read[21] << 16) | (read[22] << 8) | read[23]
        mid_gamma = (read[24] << 16) | (read[25] << 8) | read[26]
        return (
            attention,
            meditation,
            delta,
            theta,
            low_alpha,
            high_alpha,
            low_beta,
            high_beta,
            low_gamma,
            mid_gamma,
        )

    def _small_package(self):
        while a := self.ser.read():
            if a == b"\xaa":
                break
        read = self.ser.read(7)[-3:]
        raw = (read[0] << 8) | read[1]
        sum = ((0x80 + 0x02 + read[0] + read[1]) ^ 0xFFFFFFFF) & 0xFF
        if sum == read[2]:
            return raw
        else:
            return None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.ser.close()


def plot():
    import matplotlib.pyplot as plt

    i_values = [50.0]  # 存储i值的列表
    j_values = [50.0]  # 存储j值的列表
    time_values = [0]  # 存储时间的列表
    plt.ion()

    with PortReader() as reader:
        for (
            attention,
            meditation,
            delta,
            theta,
            low_alpha,
            high_alpha,
            low_beta,
            high_beta,
            low_gamma,
            mid_gamma,
        ) in reader:
            i_values.append(0.1 * attention + i_values[-1] * 0.9)  # 将i值添加到列表中
            j_values.append(0.1 * meditation + j_values[-1] * 0.9)  # 将j值添加到列表中
            if len(i_values) > 30:
                del i_values[0]
                del j_values[0]
                del time_values[0]
            time_values.append(time_values[-1] + 1)  # 将时间添加到列表中
            plt.clf()
            plt.plot(time_values, i_values, c="red")  # 绘制i值的折线图
            plt.plot(time_values, j_values, c="blue")
            plt.pause(0.5)
            plt.ioff()


plot()
