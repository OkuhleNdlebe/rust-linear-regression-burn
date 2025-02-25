# Linear Regression Model in Rust

## Overview
This project implements a simple Linear Regression model using the Burn library in Rust. It generates synthetic data, trains a model, evaluates its performance, and visualizes the results using text-based plots.

## Prerequisites
Before setting up the project, ensure you have the following installed:

- **Rust** (Install via [Rustup](https://rustup.rs/))
- **Git** (Install from [git-scm.com](https://git-scm.com/))

## Setup Instructions

1. **Clone the Repository**
   ```sh
   git clone <your-github-repo-url>
   cd linear_regression_model
   ```
2. **Ensure Cargo and Rust are Installed**
   ```sh
   cargo --version
   ```
3. **Build the Project**
   ```sh
   cargo build
   ```
4. **Run the Project**
   ```sh
   cargo run
   ```

## Approach

1. **Generating Synthetic Data**:
    - I create (x, y) pairs following the equation `y = 2x + 1` with added noise for realism.
2. **Defining the Model**:
    - A simple linear model using the Burn library is implemented.
3. **Training the Model**:
    - The model is trained using Mean Squared Error (MSE) as the loss function.
4. **Evaluating and Plotting Results**:
    - Predictions are plotted using `textplots` to visualize model accuracy.

## Challenges Faced
- Understanding how to integrate Burn's tensor operations.
- Ensuring proper tensor conversions and efficient model training.
- Debugging tensor mismatches and optimizing the learning process.

## Resources Used
- [Burn Library Documentation](https://burn.dev/docs/)
- [Rust Official Documentation](https://doc.rust-lang.org/)
- AI assistance for debugging and structuring the code
- Community discussions on Rust ML frameworks

## Future Improvements
- Implement dataset splitting into training and test sets.
- Improve model performance by tuning the optimizer parameters.
- Extend the model to support multi-variable regression.

---

Developed with 💡 in Rust!

