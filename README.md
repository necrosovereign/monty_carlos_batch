# Monty-Carlos_batch

This program is based on the [Monty-Carlos](https://github.com/necrosovereign/monty_carlos) library and can help you in the following two scenarios:

## 1) Performing Kolmogorov-Smirnov-Test OR Lilliefors-Test without the need for tables: 

Premise: You have a dataset (containing real numbers) and you have calculated the maximum vertical distance (D-max-value) between your empirical cumulative distribution function (empDistr) of this dataset and an artificially generated cumulative NORMAL distribution function (TestDistr) 
using either Kolmogorov-Smirnov-Test, where mean & standard deviation of the TestDistr are chosen on theoretical grounds, 
or Lilliefors-Test, which uses mean & standard dev. of the empirical dataset to generate the TestDistr, i.e. it "fits" the TestDistr to the empirical data. 
*Buuuuut you have a hard time finding useful tables online to tell you, what this D-max-value means in your case.* 

--> The program `monty_carlos_batch_0.1.0-x86_64-pc-windows-gnu.exe` can generate information about the fit of your empDistr to the chosen TestDistr: 
If you input the relevant specifications of your TestDistr (here: mean, standard dev), the program will generate a huge number of data samples (one "iteration" gives one data sample) from the TestDistr Monte-Carlo-style, and compare the D-max-values of all of these data samples with the D-max-value your statistical test yielded for your empirical sample. The program will then output the fraction "P" of all generated data samples that are closer to the TestDistr than your empirical sample. 
Example: P=1/3 means "a third of all generated data samples had a smaller D-max-value (i.e. were closer to the TestDistr) than the empirical data sample, two thirds had a greater D-max-value than the empirical sample. 
This value P can be interpreted as the probability that a data set that is artificially created from the TestDistr has a lower distance from the TestDistr than the empirical data set. This means: the higher this probability, the worse the fit of the empirical data to the TestDistr. 
(One minus this probability is usually denoted as "alpha" in papers about the Kolmogorov-Smirnov-Test or the Lilliefors-Test.) 

## 2) Monte-Carlo-Simulation for OP-Risk model where Poisson-Distr. models loss frequency and Log-Normal-Distr models loss amount: 

Premise: You have read Hull (2018) "Risk Management and Financial Institutions" and want to test the model for estimating operational risk from the textbook, chapter 23.3.1. 
(Brief summary: We model the frequency of loss events per year as random variable using Poisson-distribution and the loss amount per loss event as a second independent random variable using Log-Normal-distribution. Using Monte-Carlo-Simulation it is possible to generate lots of random loss amounts per year. From this data, a distribution of loss per year can be derived and with it some other information, such as VaR.) 
You have derived some mean for the Poisson-distribution and a mu and sigma for the Log-Normal-distribution and want to do a Monte-Carlo-Simulation with them, *but Excel is just too slow...* 

--> The program monty_carlos_batch_0.1.0-x86_64-pc-windows-gnu.exe can run a Monte-Carlo-Simulation producing up to (or more than) a million losses in a few seconds, and give the following types of information about the resulting data set: 

1. Option A: You will receive a list of about 101 values from the generated values (representing loss amount per year), which were picked in this way: the first and the last point are the minimum and maximum respectively of all generated values, all other values except for the last two have equally many data points between them - so between the first and second value are n more values, between the second and third values are n other values, and so on - only the gap between the second to last and last value could be a little smaller. In other words, these values are somewhat comparable to percentiles.
2. Option B: You will be given the mean and the standard deviation of the generated set of values.
3. Option C: You will be given the fraction of data points / values from the generated data set that are smaller than some value of your choice. This means, you are given the probability that your chosen value is higher, than a randomly picked value from the generated data set.
4. Option D: This is essentially the inverse of option C. You give a probability/percentage p and you will get the smallest value (loss amount per year) from the generated data set which is larger than p% of values from the same data set. This is essentially the same as getting the p-th percentile of the generated data set. If you pick p=0.95, p=0.99 or p=0.999, you can get the VaR. 

## Usage of program: see explanations in the two .toml-files pertaining to the two use-cases.

Before running the program currently named `monty_carlos_batch_0.1.0-x86_64-pc-windows-gnu.exe`, an input file of type .toml has to be created which will be called together with the program itself. 

Instructions on how to create the .toml-files can be found in the attached .toml-files: 
- "Data_fit_to_NormDistr_expl.toml" explains how to fill the .toml-file for above described use case 1).
- "Data_fit_to_NormDistr.toml" has only the relevant code, no explanations.
- "OP-Risk_Monte-Carlo_Simulation_expl.toml" explains how to fill the .toml-file for use case 2).
- "OP-Risk_Monte-Carlo_Simulation.toml" contains just the relevant code, no explanations. 

In Windows, the program has to be run via cmd.exe by inserting the following command line: 
```
[name of monty-carlos-batch-executable].exe [name of input file].toml
```
