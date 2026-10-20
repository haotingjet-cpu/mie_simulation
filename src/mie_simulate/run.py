import h5py as h5

def main():
    x = [h5.File(rf"C:\rust\mie_simulate\2000_{i}.hdf5", "r") for i in range(0,3)]

    res = [[],[]]

    for f in x:
        scattered = f['scattered'][:]
        res[0].append(scattered)

        ballistic = f['ballistic'][:]
        res[1].append(ballistic)

    scattered = [res[0][0][i] + res[0][1][i] + res[0][2][i] for i in range(3)]
    ballistic = [res[1][0][i] + res[1][1][i] + res[1][2][i] for i in range(3)]

    with h5.File("processed.hdf5", 'w') as f:
        f.create_dataset('scattered', data=scattered)
        f.create_dataset('ballistic', data=ballistic)

    for f in x :
        f.close()


main()