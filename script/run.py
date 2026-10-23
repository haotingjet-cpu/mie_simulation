import h5py as h5

def r():
    x = [h5.File(rf"C:\rust\mie_simulate\result\2000\2000_{i}.hdf5", "r") for i in range(0,2)]

    res = [[],[]]

    for f in x:
        scattered = f['scattered'][:]
        res[0].append(scattered)

        ballistic = f['ballistic'][:]
        res[1].append(ballistic)

    scattered = [res[0][0][i] + res[0][1][i]  for i in range(900)]
    ballistic = [res[1][0][i] + res[1][1][i]  for i in range(900)]

    with h5.File("l.hdf5", 'w') as f:
        f.create_dataset('scattered', data=scattered)
        f.create_dataset('ballistic', data=ballistic)

    for f in x :
        f.close()

def r2():
    with h5.File("l.hdf5", 'r') as f:
        scattered = f['scattered'][:]
        print(scattered)
r()
r2()