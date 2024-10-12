import numpy as np
import matplotlib.pyplot as plt


SAMPLERATE = 48000


DATA = [-0.002224294, 0.00461485, 0.011349851, 0.017829068, 0.02390593, 0.029440831, 0.034312226, 0.03840846, 0.041634593, 0.043919392, 0.045209642, 0.045476764, 0.04471589, 0.042942155, 0.04019762, 0.036547195, 0.032069426, 0.026866829, 0.021059223, 0.014773668, 0.008153898, 0.0013503633, -0.0054822327, -0.012189419, -0.018624406, -0.024639318, -0.030096745, -0.03487378, -0.038862497, -0.041974045, -0.044136558, -0.045300975, -0.04543945, -0.04455131, -0.042655706, -0.03979236, -0.036031816, -0.031454824, -0.026164511, -0.020285424, -0.013946853, -0.007294105, -0.00047722505, 0.0063498793, 0.013034591, 0.019424958, 0.025371758, 0.030749109, 0.035431616, 0.03931114, 0.042306118, 0.04434036, 0.04537089, 0.045375753, 0.04435361, 0.042328842, 0.039341517, 0.0354703, 0.030800046, 0.02542848, 0.01948702, 0.013103175, 0.006420471, -0.00040572367, -0.0072239502, -0.013876164, -0.020216128, -0.026099583, -0.031390723, -0.035974663, -0.039744124, -0.04261325, -0.0445204, -0.045422595, -0.045298617, -0.044150364, -0.04200426, -0.038907684, -0.034931563, -0.030165395, -0.024715526, -0.01870844, -0.012279097, -0.005570832, 0.0012637568, 0.008071152, 0.01469777, 0.020994736, 0.026819054, 0.032035533, 0.036529772, 0.040201087, 0.042962678, 0.044756602, 0.045539875, 0.045291625, 0.04402171, 0.041755788, 0.038545836, 0.034462065, 0.029595304, 0.024062878, 0.017983178, 0.011492616, 0.004742093, -0.0021206224, -0.008938344, -0.015556964, -0.021825887, -0.02760288, -0.03276002, -0.037176725, -0.04075533, -0.043415427, -0.045092765, -0.0457516, -0.04537615, -0.043971207, -0.041568216, -0.03822476, -0.034018327, -0.029044116, -0.023412727, -0.017245647, -0.010683258, -0.0038797443, 0.0030119068, 0.009838068, 0.01644337, 0.022679497, 0.028403984, 0.033486158, 0.037813757, 0.041283924, 0.043819647, 0.0453683, 0.04588844, 0.045369875, 0.043826006, 0.04128946, 0.037818976, 0.03349177, 0.028405754, 0.022675574, 0.016432833, 0.009819899, 0.002984171, -0.00392013, -0.0107382955, -0.01731322, -0.023495307, -0.02914504, -0.03413544, -0.03835496, -0.041703336, -0.04411063, -0.045520537, -0.045894913, -0.045235537, -0.043552853, -0.040880933, -0.03728453, -0.032843392, -0.02766052, -0.02185004, -0.015545276, -0.0088905925, -0.0020321768, 0.0048690797, 0.011658647, 0.018185914, 0.024301054, 0.02986714, 0.0347569, 0.038860027, 0.042081904, 0.0443508, 0.04561747, 0.04584832, 0.04504322, 0.043221634, 0.040419202, 0.036703594, 0.03215817, 0.026884515, 0.021001916, 0.014640925, 0.007951714, 0.0010827975, -0.0058133393, -0.012573222, -0.019049024, -0.02509563, -0.030571595, -0.03535589, -0.03934017, -0.04243582, -0.044572055, -0.045700163, -0.04579634, -0.044853944, -0.042898495, -0.03997405, -0.036141984, -0.031493656, -0.026132476, -0.020181676, -0.01377455, -0.007053728, -0.00017920959, 0.0067034424, 0.013436278, 0.019855483, 0.025829071, 0.031219682, 0.03590067, 0.039772343, 0.04274562, 0.044755165, 0.04574971, 0.045708958, 0.044639032, 0.0425547, 0.03950984, 0.035574343, 0.030829966, 0.025389817, 0.019374745, 0.01292017, 0.0061723506, -0.00071764557, -0.0075885425, -0.014287228, -0.020664925, -0.026575288, -0.03188316, -0.03646802, -0.040227428, -0.04307624, -0.044951964, -0.045810357, -0.04563144, -0.044423338, -0.042210232, -0.039042477, -0.034993626, -0.030152394, -0.024628412, -0.018547775, -0.012048241, -0.0052754544, 0.001616092, 0.008472424, 0.015140912, 0.021464046, 0.027301433, 0.032524098, 0.037008878, 0.040657856, 0.043389045, 0.045138013, 0.04586553, 0.04555486, 0.044213828, 0.041869674, 0.03857733, 0.03441386, 0.029469576, 0.023860076, 0.017712455, 0.011162552, 0.0043605003, -0.0025414, -0.00938396, -0.016012264, -0.022279305, -0.028038867, -0.033165198, -0.037544336, -0.041072257, -0.04367127, -0.045282017, -0.0458684, -0.045417413, -0.04393949, -0.041470163, -0.038060084, -0.03378929, -0.028757492, -0.02307243, -0.016865876, -0.010279408, -0.0034600333, 0.0034364837, 0.010255698, 0.016842715, 0.02304907, 0.028734224, 0.033767644, 0.03803806, 0.041448418, 0.043920975, 0.04540067, 0.04585277, 0.045267962, 0.0436585, 0.04106127, 0.03753488, 0.033156153, 0.028030142, 0.022271786, 0.016006408, 0.009381698, 0.0025450706, -0.0043502767, -0.011146723, -0.017691547, -0.023831818, -0.02943485, -0.034373745, -0.038530555, -0.04181931, -0.044161513, -0.045501225, -0.045813352, -0.04508912, -0.04334473, -0.040618245, -0.036974795, -0.032496013, -0.027281648, -0.02145447, -0.015138584, -0.008478445, -0.0016302649, 0.005255643, 0.012021181, 0.018514466, 0.024588376, 0.030106362, 0.0349464, 0.038992945, 0.04215747, 0.044370767, 0.04557935, 0.045758307, 0.044902775, 0.0430314, 0.040186793, 0.03643275, 0.031854626, 0.026554383, 0.02065255, 0.01428493, 0.0075945086, 0.0007315324, -0.0061481907, -0.012890249, -0.019341547, -0.025354842, -0.030794175, -0.035535306, -0.03947343, -0.042519115, -0.044601593, -0.045675706, -0.045715004, -0.044719122, -0.04271199, -0.03973841, -0.035865564, -0.0311797, -0.02579007, -0.019817637, -0.013396274, -0.0066731595, 0.0002033553, 0.0070753903, 0.01378521, 0.020182457, 0.026122727, 0.031472407, 0.036108762, 0.03992946, 0.042845316, 0.044789877, 0.04572578, 0.04562581, 0.04449262, 0.04235539, 0.039259195, 0.035275947, 0.03049426, 0.02502292, 0.018983353, 0.012512455, 0.0057638553, -0.0011163389, -0.007971743, -0.014643966, -0.020988269, -0.026857538, -0.032118548, -0.03665203, -0.040354684, -0.043146852, -0.044961628, -0.04575916, -0.045523662, -0.044257496, -0.041990373, -0.03877274, -0.03467739, -0.029796915, -0.024242653, -0.018142335, -0.011630276, -0.004856041, 0.0020260855, 0.008866215, 0.01550534, 0.021790477, 0.027583102, 0.032750968, 0.037177656, 0.040764008, 0.043428656, 0.045112252, 0.045775093, 0.045402154, 0.04400148, 0.041604422, 0.03826738, 0.0340643, 0.029090706, 0.02345908, 0.017296014, 0.010743284, 0.0039455863, -0.0029407616, -0.009756642, -0.016353685, -0.022580367, -0.028295582, -0.03337268, -0.037694484, -0.041163113, -0.043698903, -0.04524717, -0.04577162, -0.04525734, -0.043722022, -0.041197818, -0.03774018, -0.033431128, -0.028365057, -0.022657055, -0.016437117, -0.009846609, -0.003034945, 0.0038463818, 0.010639302, 0.017192665, 0.023359917, 0.028996274, 0.03397607, 0.038187485, 0.0415347, 0.04394322, 0.045356706, 0.04574495, 0.04509825, 0.043430097, 0.04078095, 0.037207767, 0.03279151, 0.02763441, 0.02185287, 0.015578315, 0.008950678, 0.0021199065, -0.0047600036, -0.01153262, -0.018040642, -0.0241399, -0.029693406, -0.034574907, -0.038675796, -0.04190144, -0.044178486, -0.045454953, -0.045703888, -0.044919454, -0.043116555, -0.04034043, -0.036651723, -0.032132495, -0.026889468, -0.02103744, -0.014707879, -0.008045515, -0.0012020972, 0.00566705, 0.012408193, 0.018866984, 0.024901245, 0.030373825, 0.035155363, 0.039142765, 0.0422452, 0.044391103, 0.045533974]




print("Loading FFT data...")
fft_result = np.fft.fft(DATA)
print("Solving frequencies...")
frequencies = np.fft.fftfreq(len(DATA), 1/SAMPLERATE)

print("Building graph...")
plt.plot(frequencies, np.abs(fft_result))
plt.xlabel("Frequency (Hz)")
plt.ylabel("Magnitude")
plt.title("FFT of Provided Signal")
plt.show()