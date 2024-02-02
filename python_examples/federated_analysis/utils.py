import numpy as np
from numpy.random import RandomState

# Create the data
def createData(N, mu, sigma):
    rng = RandomState(2)
    # return np.random.randn(N)*sigma + mu
    return rng.randn(N)*sigma + mu

# Simulate reading the data from local database
def getData(treatment):

    # Number of observations for the different regions
    N = int((1298-357)/7) if treatment==0 else int(357/7)
    
    # Mean of distribution. 
    # To emulate some real-life scenario, the cohort without treatment(=0) might have a higher mean
    mu = 80 + (10 if treatment==0 else 0)

    # Standardeviation of the cohort, the cohort without treatment(=0) might have a higher variance
    sigma = 40 + (20 if treatment==0 else 0)
    
    return createData(N, mu, sigma)
