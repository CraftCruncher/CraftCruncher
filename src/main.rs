mod fetcher;
mod resolver;
mod categorizer;

fn main() {
    fetcher::runner::run();
    resolver::runner::run();
    categorizer::runner::run();
}