Project Description.

The idea of the project is evolution and natural selection of "bacteria" due to the changing genotype. Only bacteria which were able to accumulate energy can create new bacteria. Each time a new bacterium divides, the genome is slightly different from that of its ancestor, so the changes will never stop!


Genotype...
You may have seen similar projects before. The idea behind my project is a genotype of limited size and different proportions of genes in each bacterium.
Let's say the entire genotype is 100% and we only have 5 possible genes. Then the bacterium can fill its genotype with these 5 genes in equal proportions, or it can leave only 1 or 2 genes.

Example bacteria:
Bacteria "Aphrodite".
   - photosynthesis - 30%
   - movement speed - 20%
   - defense - 40%
   - reproduction rate - 10%
   - carnivorousness - 0%

Bacterium "T-800 CSM 101"
   - photosynthesis - 0%
   - transport speed - 10%
   - defense - 10%
   - reproduction rate - 0% (oh nooo!)
   - carnivorousness - 80%
   
   
Colors...
The color reflects the way the bacteria feed. Right now we have 3 types of nutrition:
   - photosynthesis - green
   - carnivorous - red
   - saprophyte - yellow
The color is mixed on the RGB channel depending on the manifestation of the genes. For example, if you see a purple bacterium, it has developed carnivory and saprophyte!


The logic is processed in ECS style.
All genes are just a float array, and the index is a bacteria. (See bacteries_processing.rs)


The live::save_load::try_save() method saves the current state of the bacteria to a .csv file, so you can view the state of the bacteria in the table and make a graph.


The first time you run the application, it creates a file "bac_settings.txt" in the current directory, where you can experiment with the simulation settings.
